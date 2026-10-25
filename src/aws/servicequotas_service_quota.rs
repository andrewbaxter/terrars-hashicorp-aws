#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ServicequotasServiceQuotaData_ {
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
    quota_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_code: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

struct ServicequotasServiceQuota_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ServicequotasServiceQuotaData_>,
}

#[derive(Clone)]
pub struct ServicequotasServiceQuota(std::rc::Rc<ServicequotasServiceQuota_>);

impl ServicequotasServiceQuota {
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

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> terrars::ListRef<ServicequotasServiceQuota_UsageMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl terrars::Referable for ServicequotasServiceQuota {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ServicequotasServiceQuota { }

impl terrars::ToListMappable for ServicequotasServiceQuota {
    type O = terrars::ListRef<ServicequotasServiceQuotaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ServicequotasServiceQuota_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicequotasServiceQuota {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub quota_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildServicequotasServiceQuota {
    pub fn build(self, stack: &mut terrars::Stack) -> ServicequotasServiceQuota {
        let out = ServicequotasServiceQuota(std::rc::Rc::new(ServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ServicequotasServiceQuotaData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                quota_code: self.quota_code,
                region: core::default::Default::default(),
                service_code: self.service_code,
                value: self.value,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicequotasServiceQuotaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicequotasServiceQuotaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServicequotasServiceQuotaRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\n"]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_status` after provisioning.\n"]
    pub fn request_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> terrars::ListRef<ServicequotasServiceQuota_UsageMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ServicequotasServiceQuota_UsageMetric_MetricDimensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServicequotasServiceQuota_UsageMetric_MetricDimensions {
    #[doc = "Set the field `class`.\n"]
    pub fn set_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicequotasServiceQuota_UsageMetric_MetricDimensions {
    type O = terrars::BlockAssignable<ServicequotasServiceQuota_UsageMetric_MetricDimensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicequotasServiceQuota_UsageMetric_MetricDimensions {}

impl BuildServicequotasServiceQuota_UsageMetric_MetricDimensions {
    pub fn build(self) -> ServicequotasServiceQuota_UsageMetric_MetricDimensions {
        ServicequotasServiceQuota_UsageMetric_MetricDimensions {
            class: core::default::Default::default(),
            resource: core::default::Default::default(),
            service: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
        ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `class` after provisioning.\n"]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ServicequotasServiceQuota_UsageMetric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_dimensions: std::option::Option<terrars::ListField<ServicequotasServiceQuota_UsageMetric_MetricDimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_statistic_recommendation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServicequotasServiceQuota_UsageMetric {
    #[doc = "Set the field `metric_dimensions`.\n"]
    pub fn set_metric_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ServicequotasServiceQuota_UsageMetric_MetricDimensions>>,
    ) -> Self {
        self.metric_dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_namespace`.\n"]
    pub fn set_metric_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_statistic_recommendation`.\n"]
    pub fn set_metric_statistic_recommendation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_statistic_recommendation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicequotasServiceQuota_UsageMetric {
    type O = terrars::BlockAssignable<ServicequotasServiceQuota_UsageMetric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicequotasServiceQuota_UsageMetric {}

impl BuildServicequotasServiceQuota_UsageMetric {
    pub fn build(self) -> ServicequotasServiceQuota_UsageMetric {
        ServicequotasServiceQuota_UsageMetric {
            metric_dimensions: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            metric_namespace: core::default::Default::default(),
            metric_statistic_recommendation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ServicequotasServiceQuota_UsageMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicequotasServiceQuota_UsageMetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServicequotasServiceQuota_UsageMetricRef {
        ServicequotasServiceQuota_UsageMetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicequotasServiceQuota_UsageMetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_dimensions` after provisioning.\n"]
    pub fn metric_dimensions(&self) -> terrars::ListRef<ServicequotasServiceQuota_UsageMetric_MetricDimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_statistic_recommendation` after provisioning.\n"]
    pub fn metric_statistic_recommendation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_statistic_recommendation", self.base))
    }
}
