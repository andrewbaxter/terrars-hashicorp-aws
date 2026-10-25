#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingplansScalingPlanData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_source: std::option::Option<std::vec::Vec<AutoscalingplansScalingPlan_ApplicationSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_instruction: std::option::Option<std::vec::Vec<AutoscalingplansScalingPlan_ScalingInstruction>>,
    dynamic: AutoscalingplansScalingPlanDynamic,
}

struct AutoscalingplansScalingPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AutoscalingplansScalingPlanData_>,
}

#[derive(Clone)]
pub struct AutoscalingplansScalingPlan(std::rc::Rc<AutoscalingplansScalingPlan_>);

impl AutoscalingplansScalingPlan {
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

    #[doc = "Set the field `application_source`.\n"]
    pub fn set_application_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingplansScalingPlan_ApplicationSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_instruction`.\n"]
    pub fn set_scaling_instruction(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_instruction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_instruction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_plan_version` after provisioning.\n"]
    pub fn scaling_plan_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_plan_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_source` after provisioning.\n"]
    pub fn application_source(&self) -> terrars::ListRef<AutoscalingplansScalingPlan_ApplicationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }
}

impl terrars::Referable for AutoscalingplansScalingPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AutoscalingplansScalingPlan { }

impl terrars::ToListMappable for AutoscalingplansScalingPlan {
    type O = terrars::ListRef<AutoscalingplansScalingPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AutoscalingplansScalingPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_autoscalingplans_scaling_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingplansScalingPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> AutoscalingplansScalingPlan {
        let out = AutoscalingplansScalingPlan(std::rc::Rc::new(AutoscalingplansScalingPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AutoscalingplansScalingPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                application_source: core::default::Default::default(),
                scaling_instruction: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingplansScalingPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AutoscalingplansScalingPlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_plan_version` after provisioning.\n"]
    pub fn scaling_plan_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_plan_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_source` after provisioning.\n"]
    pub fn application_source(&self) -> terrars::ListRef<AutoscalingplansScalingPlan_ApplicationSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlanDynamic {
    application_source: std::option::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ApplicationSource>>,
    scaling_instruction: std::option::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ScalingInstruction>>,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ApplicationSource_TagFilter {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl AutoscalingplansScalingPlan_ApplicationSource_TagFilter {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ApplicationSource_TagFilter {
    type O = terrars::BlockAssignable<AutoscalingplansScalingPlan_ApplicationSource_TagFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ApplicationSource_TagFilter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ApplicationSource_TagFilter {
    pub fn build(self) -> AutoscalingplansScalingPlan_ApplicationSource_TagFilter {
        AutoscalingplansScalingPlan_ApplicationSource_TagFilter {
            key: self.key,
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ApplicationSource_TagFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ApplicationSource_TagFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ApplicationSource_TagFilterRef {
        AutoscalingplansScalingPlan_ApplicationSource_TagFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ApplicationSource_TagFilterRef {
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
struct AutoscalingplansScalingPlan_ApplicationSourceDynamic {
    tag_filter: std::option::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ApplicationSource_TagFilter>>,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ApplicationSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudformation_stack_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_filter: std::option::Option<std::vec::Vec<AutoscalingplansScalingPlan_ApplicationSource_TagFilter>>,
    dynamic: AutoscalingplansScalingPlan_ApplicationSourceDynamic,
}

impl AutoscalingplansScalingPlan_ApplicationSource {
    #[doc = "Set the field `cloudformation_stack_arn`.\n"]
    pub fn set_cloudformation_stack_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudformation_stack_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_filter`.\n"]
    pub fn set_tag_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AutoscalingplansScalingPlan_ApplicationSource_TagFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ApplicationSource {
    type O = terrars::BlockAssignable<AutoscalingplansScalingPlan_ApplicationSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ApplicationSource {}

impl BuildAutoscalingplansScalingPlan_ApplicationSource {
    pub fn build(self) -> AutoscalingplansScalingPlan_ApplicationSource {
        AutoscalingplansScalingPlan_ApplicationSource {
            cloudformation_stack_arn: core::default::Default::default(),
            tag_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ApplicationSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ApplicationSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ApplicationSourceRef {
        AutoscalingplansScalingPlan_ApplicationSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ApplicationSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudformation_stack_arn` after provisioning.\n"]
    pub fn cloudformation_stack_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudformation_stack_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    statistic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
    type O =
        terrars::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub statistic: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
    pub fn build(self) -> AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
        AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef {
        AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
    predefined_load_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
    type O =
        terrars::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
    #[doc = ""]
    pub predefined_load_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
    pub fn build(self) -> AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
        AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification {
            predefined_load_metric_type: self.predefined_load_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef {
        AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_type` after provisioning.\n"]
    pub fn predefined_load_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_load_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    metric_name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    statistic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub statistic: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
    predefined_scaling_metric_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
    type O =
        terrars
        ::BlockAssignable<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
    #[doc = ""]
    pub predefined_scaling_metric_type: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
    pub fn build(
        self,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification {
            predefined_scaling_metric_type: self.predefined_scaling_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_type` after provisioning.\n"]
    pub fn predefined_scaling_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_scaling_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationDynamic {
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification,
        >,
    >,
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_scale_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_instance_warmup: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_cooldown: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_out_cooldown: std::option::Option<terrars::PrimField<f64>>,
    target_value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_scaling_metric_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification,
        >,
    >,
    dynamic: AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationDynamic,
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_scale_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.estimated_instance_warmup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_cooldown`.\n"]
    pub fn set_scale_in_cooldown(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in_cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_cooldown`.\n"]
    pub fn set_scale_out_cooldown(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_out_cooldown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customized_scaling_metric_specification`.\n"]
    pub fn set_customized_scaling_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_scaling_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_scaling_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_scaling_metric_specification`.\n"]
    pub fn set_predefined_scaling_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_scaling_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_scaling_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
    type O = terrars::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
    pub fn build(self) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration {
            disable_scale_in: core::default::Default::default(),
            estimated_instance_warmup: core::default::Default::default(),
            scale_in_cooldown: core::default::Default::default(),
            scale_out_cooldown: core::default::Default::default(),
            target_value: self.target_value,
            customized_scaling_metric_specification: core::default::Default::default(),
            predefined_scaling_metric_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationRef {
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown` after provisioning.\n"]
    pub fn scale_in_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown` after provisioning.\n"]
    pub fn scale_out_cooldown(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_CustomizedScalingMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customized_scaling_metric_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> terrars
    ::ListRef<
        AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration_PredefinedScalingMetricSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_scaling_metric_specification", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlan_ScalingInstructionDynamic {
    customized_load_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification>>,
    predefined_load_metric_specification: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification>>,
    target_tracking_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration>>,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlan_ScalingInstruction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_dynamic_scaling: std::option::Option<terrars::PrimField<bool>>,
    max_capacity: terrars::PrimField<f64>,
    min_capacity: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_max_capacity_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_max_capacity_buffer: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predictive_scaling_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_id: terrars::PrimField<std::string::String>,
    scalable_dimension: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_policy_update_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_action_buffer_time: std::option::Option<terrars::PrimField<f64>>,
    service_namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customized_load_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_load_metric_specification: std
    ::option
    ::Option<std::vec::Vec<AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_configuration: std
    ::option
    ::Option<std::vec::Vec<AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration>>,
    dynamic: AutoscalingplansScalingPlan_ScalingInstructionDynamic,
}

impl AutoscalingplansScalingPlan_ScalingInstruction {
    #[doc = "Set the field `disable_dynamic_scaling`.\n"]
    pub fn set_disable_dynamic_scaling(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_dynamic_scaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_max_capacity_behavior`.\n"]
    pub fn set_predictive_scaling_max_capacity_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.predictive_scaling_max_capacity_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_max_capacity_buffer`.\n"]
    pub fn set_predictive_scaling_max_capacity_buffer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.predictive_scaling_max_capacity_buffer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_mode`.\n"]
    pub fn set_predictive_scaling_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.predictive_scaling_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_policy_update_behavior`.\n"]
    pub fn set_scaling_policy_update_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scaling_policy_update_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_action_buffer_time`.\n"]
    pub fn set_scheduled_action_buffer_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scheduled_action_buffer_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customized_load_metric_specification`.\n"]
    pub fn set_customized_load_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customized_load_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_load_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_load_metric_specification`.\n"]
    pub fn set_predefined_load_metric_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predefined_load_metric_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_load_metric_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction_TargetTrackingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_tracking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AutoscalingplansScalingPlan_ScalingInstruction {
    type O = terrars::BlockAssignable<AutoscalingplansScalingPlan_ScalingInstruction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlan_ScalingInstruction {
    #[doc = ""]
    pub max_capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub resource_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scalable_dimension: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_namespace: terrars::PrimField<std::string::String>,
}

impl BuildAutoscalingplansScalingPlan_ScalingInstruction {
    pub fn build(self) -> AutoscalingplansScalingPlan_ScalingInstruction {
        AutoscalingplansScalingPlan_ScalingInstruction {
            disable_dynamic_scaling: core::default::Default::default(),
            max_capacity: self.max_capacity,
            min_capacity: self.min_capacity,
            predictive_scaling_max_capacity_behavior: core::default::Default::default(),
            predictive_scaling_max_capacity_buffer: core::default::Default::default(),
            predictive_scaling_mode: core::default::Default::default(),
            resource_id: self.resource_id,
            scalable_dimension: self.scalable_dimension,
            scaling_policy_update_behavior: core::default::Default::default(),
            scheduled_action_buffer_time: core::default::Default::default(),
            service_namespace: self.service_namespace,
            customized_load_metric_specification: core::default::Default::default(),
            predefined_load_metric_specification: core::default::Default::default(),
            target_tracking_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AutoscalingplansScalingPlan_ScalingInstructionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AutoscalingplansScalingPlan_ScalingInstructionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AutoscalingplansScalingPlan_ScalingInstructionRef {
        AutoscalingplansScalingPlan_ScalingInstructionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlan_ScalingInstructionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_dynamic_scaling` after provisioning.\n"]
    pub fn disable_dynamic_scaling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_dynamic_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_max_capacity_behavior` after provisioning.\n"]
    pub fn predictive_scaling_max_capacity_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_max_capacity_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_max_capacity_buffer` after provisioning.\n"]
    pub fn predictive_scaling_max_capacity_buffer(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_max_capacity_buffer", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_mode` after provisioning.\n"]
    pub fn predictive_scaling_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predictive_scaling_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_policy_update_behavior` after provisioning.\n"]
    pub fn scaling_policy_update_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_policy_update_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_action_buffer_time` after provisioning.\n"]
    pub fn scheduled_action_buffer_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_action_buffer_time", self.base))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_load_metric_specification` after provisioning.\n"]
    pub fn customized_load_metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingplansScalingPlan_ScalingInstruction_CustomizedLoadMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_specification` after provisioning.\n"]
    pub fn predefined_load_metric_specification(
        &self,
    ) -> terrars::ListRef<AutoscalingplansScalingPlan_ScalingInstruction_PredefinedLoadMetricSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }
}
