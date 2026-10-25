#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AccessanalyzerAnalyzerData_ {
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
    analyzer_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration>>,
    dynamic: AccessanalyzerAnalyzerDynamic,
}

struct AccessanalyzerAnalyzer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessanalyzerAnalyzerData_>,
}

#[derive(Clone)]
pub struct AccessanalyzerAnalyzer(std::rc::Rc<AccessanalyzerAnalyzer_>);

impl AccessanalyzerAnalyzer {
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyzer_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for AccessanalyzerAnalyzer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessanalyzerAnalyzer { }

impl terrars::ToListMappable for AccessanalyzerAnalyzer {
    type O = terrars::ListRef<AccessanalyzerAnalyzerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessanalyzerAnalyzer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_accessanalyzer_analyzer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessanalyzerAnalyzer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub analyzer_name: terrars::PrimField<std::string::String>,
}

impl BuildAccessanalyzerAnalyzer {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessanalyzerAnalyzer {
        let out = AccessanalyzerAnalyzer(std::rc::Rc::new(AccessanalyzerAnalyzer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessanalyzerAnalyzerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                analyzer_name: self.analyzer_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessanalyzerAnalyzerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessanalyzerAnalyzerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyzer_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
    #[doc = "Set the field `account_ids`.\n"]
    pub fn set_account_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.account_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {}

impl BuildAccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
        AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion {
            account_ids: core::default::Default::default(),
            resource_arns: core::default::Default::default(),
            resource_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef {
        AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleDynamic {
    inclusion: std
    ::option
    ::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion: std
    ::option
    ::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion>>,
    dynamic: AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleDynamic,
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
    #[doc = "Set the field `inclusion`.\n"]
    pub fn set_inclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_Inclusion>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inclusion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inclusion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {}

impl BuildAccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
        AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule {
            inclusion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef {
        AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inclusion` after provisioning.\n"]
    pub fn inclusion(
        &self,
    ) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule_InclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inclusion", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzer_Configuration_InternalAccessDynamic {
    analysis_rule: std
    ::option
    ::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis_rule: std::option::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule>>,
    dynamic: AccessanalyzerAnalyzer_Configuration_InternalAccessDynamic,
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccess {
    #[doc = "Set the field `analysis_rule`.\n"]
    pub fn set_analysis_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_InternalAccess {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_InternalAccess {}

impl BuildAccessanalyzerAnalyzer_Configuration_InternalAccess {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_InternalAccess {
        AccessanalyzerAnalyzer_Configuration_InternalAccess {
            analysis_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_InternalAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_InternalAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_InternalAccessRef {
        AccessanalyzerAnalyzer_Configuration_InternalAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_InternalAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analysis_rule` after provisioning.\n"]
    pub fn analysis_rule(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_InternalAccess_AnalysisRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
    #[doc = "Set the field `account_ids`.\n"]
    pub fn set_account_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.account_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.resource_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {}

impl BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
        AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion {
            account_ids: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef {
        AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleDynamic {
    exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std
    ::option
    ::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion>>,
    dynamic: AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleDynamic,
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_Exclusion>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {}

impl BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
        AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule {
            exclusion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef {
        AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzer_Configuration_UnusedAccessDynamic {
    analysis_rule: std
    ::option
    ::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unused_access_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis_rule: std::option::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule>>,
    dynamic: AccessanalyzerAnalyzer_Configuration_UnusedAccessDynamic,
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccess {
    #[doc = "Set the field `unused_access_age`.\n"]
    pub fn set_unused_access_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unused_access_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis_rule`.\n"]
    pub fn set_analysis_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration_UnusedAccess {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess {}

impl BuildAccessanalyzerAnalyzer_Configuration_UnusedAccess {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration_UnusedAccess {
        AccessanalyzerAnalyzer_Configuration_UnusedAccess {
            unused_access_age: core::default::Default::default(),
            analysis_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_Configuration_UnusedAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_Configuration_UnusedAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessanalyzerAnalyzer_Configuration_UnusedAccessRef {
        AccessanalyzerAnalyzer_Configuration_UnusedAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_Configuration_UnusedAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unused_access_age` after provisioning.\n"]
    pub fn unused_access_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unused_access_age", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis_rule` after provisioning.\n"]
    pub fn analysis_rule(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_UnusedAccess_AnalysisRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzer_ConfigurationDynamic {
    internal_access: std::option::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_InternalAccess>>,
    unused_access: std::option::Option<terrars::DynamicBlock<AccessanalyzerAnalyzer_Configuration_UnusedAccess>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzer_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_access: std::option::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_InternalAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unused_access: std::option::Option<std::vec::Vec<AccessanalyzerAnalyzer_Configuration_UnusedAccess>>,
    dynamic: AccessanalyzerAnalyzer_ConfigurationDynamic,
}

impl AccessanalyzerAnalyzer_Configuration {
    #[doc = "Set the field `internal_access`.\n"]
    pub fn set_internal_access(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_InternalAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unused_access`.\n"]
    pub fn set_unused_access(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration_UnusedAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unused_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unused_access = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessanalyzerAnalyzer_Configuration {
    type O = terrars::BlockAssignable<AccessanalyzerAnalyzer_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzer_Configuration {}

impl BuildAccessanalyzerAnalyzer_Configuration {
    pub fn build(self) -> AccessanalyzerAnalyzer_Configuration {
        AccessanalyzerAnalyzer_Configuration {
            internal_access: core::default::Default::default(),
            unused_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessanalyzerAnalyzer_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessanalyzerAnalyzer_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessanalyzerAnalyzer_ConfigurationRef {
        AccessanalyzerAnalyzer_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzer_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_access` after provisioning.\n"]
    pub fn internal_access(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_InternalAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_access", self.base))
    }

    #[doc = "Get a reference to the value of field `unused_access` after provisioning.\n"]
    pub fn unused_access(&self) -> terrars::ListRef<AccessanalyzerAnalyzer_Configuration_UnusedAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unused_access", self.base))
    }
}
