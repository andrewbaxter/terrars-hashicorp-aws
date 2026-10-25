#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Resiliencehubv2InputSourceData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration: std::option::Option<std::vec::Vec<Resiliencehubv2InputSource_ResourceConfiguration>>,
    dynamic: Resiliencehubv2InputSourceDynamic,
}

struct Resiliencehubv2InputSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Resiliencehubv2InputSourceData_>,
}

#[derive(Clone)]
pub struct Resiliencehubv2InputSource(std::rc::Rc<Resiliencehubv2InputSource_>);

impl Resiliencehubv2InputSource {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration`.\n"]
    pub fn set_resource_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `input_source_id` after provisioning.\n"]
    pub fn input_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration` after provisioning.\n"]
    pub fn resource_configuration(&self) -> terrars::ListRef<Resiliencehubv2InputSource_ResourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for Resiliencehubv2InputSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Resiliencehubv2InputSource { }

impl terrars::ToListMappable for Resiliencehubv2InputSource {
    type O = terrars::ListRef<Resiliencehubv2InputSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Resiliencehubv2InputSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_resiliencehubv2_input_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResiliencehubv2InputSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub service_arn: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubv2InputSource {
    pub fn build(self, stack: &mut terrars::Stack) -> Resiliencehubv2InputSource {
        let out = Resiliencehubv2InputSource(std::rc::Rc::new(Resiliencehubv2InputSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Resiliencehubv2InputSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                service_arn: self.service_arn,
                resource_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Resiliencehubv2InputSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Resiliencehubv2InputSourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_source_id` after provisioning.\n"]
    pub fn input_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration` after provisioning.\n"]
    pub fn resource_configuration(&self) -> terrars::ListRef<Resiliencehubv2InputSource_ResourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Resiliencehubv2InputSourceDynamic {
    resource_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Resiliencehubv2InputSource_ResourceConfiguration>>,
}

#[derive(Serialize)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
    type O =
        terrars
        ::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
    pub fn build(self) -> Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
        Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions {
            key: self.key,
            operator: self.operator,
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressionsRef {
        Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorDynamic {
    match_expressions: std
    ::option
    ::Option<
        terrars::DynamicBlock<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions>,
    >,
}

#[derive(Serialize)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_expressions: std
    ::option
    ::Option<std::vec::Vec<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions>>,
    dynamic: Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorDynamic,
}

impl Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
    #[doc = "Set the field `match_labels`.\n"]
    pub fn set_match_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_expressions`.\n"]
    pub fn set_match_expressions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector_MatchExpressions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_expressions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_expressions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
    type O = terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {}

impl BuildResiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
    pub fn build(self) -> Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
        Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector {
            match_labels: core::default::Default::default(),
            match_expressions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef {
        Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_labels` after provisioning.\n"]
    pub fn match_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.match_labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct Resiliencehubv2InputSource_ResourceConfiguration_EksDynamic {
    label_selector: std
    ::option
    ::Option<terrars::DynamicBlock<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector>>,
}

#[derive(Serialize)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_Eks {
    cluster_arn: terrars::PrimField<std::string::String>,
    namespaces: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_selector: std
    ::option
    ::Option<std::vec::Vec<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector>>,
    dynamic: Resiliencehubv2InputSource_ResourceConfiguration_EksDynamic,
}

impl Resiliencehubv2InputSource_ResourceConfiguration_Eks {
    #[doc = "Set the field `label_selector`.\n"]
    pub fn set_label_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelector>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2InputSource_ResourceConfiguration_Eks {
    type O = terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_Eks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2InputSource_ResourceConfiguration_Eks {
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespaces: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildResiliencehubv2InputSource_ResourceConfiguration_Eks {
    pub fn build(self) -> Resiliencehubv2InputSource_ResourceConfiguration_Eks {
        Resiliencehubv2InputSource_ResourceConfiguration_Eks {
            cluster_arn: self.cluster_arn,
            namespaces: self.namespaces,
            label_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_EksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSource_ResourceConfiguration_EksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2InputSource_ResourceConfiguration_EksRef {
        Resiliencehubv2InputSource_ResourceConfiguration_EksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2InputSource_ResourceConfiguration_EksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `label_selector` after provisioning.\n"]
    pub fn label_selector(
        &self,
    ) -> terrars::ListRef<Resiliencehubv2InputSource_ResourceConfiguration_Eks_LabelSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag { }

impl terrars::ToListMappable for Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
    type O = terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildResiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
    pub fn build(self) -> Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
        Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration_ResourceTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSource_ResourceConfiguration_ResourceTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2InputSource_ResourceConfiguration_ResourceTagRef {
        Resiliencehubv2InputSource_ResourceConfiguration_ResourceTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2InputSource_ResourceConfiguration_ResourceTagRef {
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

#[derive(Serialize, Default)]
struct Resiliencehubv2InputSource_ResourceConfigurationDynamic {
    eks: std::option::Option<terrars::DynamicBlock<Resiliencehubv2InputSource_ResourceConfiguration_Eks>>,
    resource_tag: std
    ::option
    ::Option<terrars::DynamicBlock<Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag>>,
}

#[derive(Serialize)]
pub struct Resiliencehubv2InputSource_ResourceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cfn_stack_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    design_file_s3_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tf_state_file_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks: std::option::Option<std::vec::Vec<Resiliencehubv2InputSource_ResourceConfiguration_Eks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag: std::option::Option<std::vec::Vec<Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag>>,
    dynamic: Resiliencehubv2InputSource_ResourceConfigurationDynamic,
}

impl Resiliencehubv2InputSource_ResourceConfiguration {
    #[doc = "Set the field `cfn_stack_arn`.\n"]
    pub fn set_cfn_stack_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cfn_stack_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `design_file_s3_url`.\n"]
    pub fn set_design_file_s3_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.design_file_s3_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tf_state_file_url`.\n"]
    pub fn set_tf_state_file_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tf_state_file_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `eks`.\n"]
    pub fn set_eks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_Eks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tag`.\n"]
    pub fn set_resource_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration_ResourceTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2InputSource_ResourceConfiguration {
    type O = terrars::BlockAssignable<Resiliencehubv2InputSource_ResourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2InputSource_ResourceConfiguration {}

impl BuildResiliencehubv2InputSource_ResourceConfiguration {
    pub fn build(self) -> Resiliencehubv2InputSource_ResourceConfiguration {
        Resiliencehubv2InputSource_ResourceConfiguration {
            cfn_stack_arn: core::default::Default::default(),
            design_file_s3_url: core::default::Default::default(),
            tf_state_file_url: core::default::Default::default(),
            eks: core::default::Default::default(),
            resource_tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2InputSource_ResourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2InputSource_ResourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2InputSource_ResourceConfigurationRef {
        Resiliencehubv2InputSource_ResourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2InputSource_ResourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cfn_stack_arn` after provisioning.\n"]
    pub fn cfn_stack_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cfn_stack_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `design_file_s3_url` after provisioning.\n"]
    pub fn design_file_s3_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.design_file_s3_url", self.base))
    }

    #[doc = "Get a reference to the value of field `tf_state_file_url` after provisioning.\n"]
    pub fn tf_state_file_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tf_state_file_url", self.base))
    }

    #[doc = "Get a reference to the value of field `eks` after provisioning.\n"]
    pub fn eks(&self) -> terrars::ListRef<Resiliencehubv2InputSource_ResourceConfiguration_EksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks", self.base))
    }
}
