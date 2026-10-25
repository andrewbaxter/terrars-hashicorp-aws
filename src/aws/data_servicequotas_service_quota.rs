#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicequotasServiceQuotaData_ {
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
    quota_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_code: terrars::PrimField<std::string::String>,
}

struct DataServicequotasServiceQuota_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServicequotasServiceQuotaData_>,
}

#[derive(Clone)]
pub struct DataServicequotasServiceQuota(std::rc::Rc<DataServicequotasServiceQuota_>);

impl DataServicequotasServiceQuota {
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

    #[doc = "Set the field `quota_code`.\n"]
    pub fn set_quota_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_name`.\n"]
    pub fn set_quota_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_name = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> terrars::ListRef<DataServicequotasServiceQuota_UsageMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl terrars::Referable for DataServicequotasServiceQuota {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServicequotasServiceQuota { }

impl terrars::ToListMappable for DataServicequotasServiceQuota {
    type O = terrars::ListRef<DataServicequotasServiceQuotaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServicequotasServiceQuota_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicequotasServiceQuota {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub service_code: terrars::PrimField<std::string::String>,
}

impl BuildDataServicequotasServiceQuota {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServicequotasServiceQuota {
        let out = DataServicequotasServiceQuota(std::rc::Rc::new(DataServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServicequotasServiceQuotaData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                quota_code: core::default::Default::default(),
                quota_name: core::default::Default::default(),
                region: core::default::Default::default(),
                service_code: self.service_code,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicequotasServiceQuotaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicequotasServiceQuotaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServicequotasServiceQuotaRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_metric` after provisioning.\n"]
    pub fn usage_metric(&self) -> terrars::ListRef<DataServicequotasServiceQuota_UsageMetricRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServicequotasServiceQuota_UsageMetric_MetricDimensions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicequotasServiceQuota_UsageMetric_MetricDimensions {
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

impl terrars::ToListMappable for DataServicequotasServiceQuota_UsageMetric_MetricDimensions {
    type O = terrars::BlockAssignable<DataServicequotasServiceQuota_UsageMetric_MetricDimensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicequotasServiceQuota_UsageMetric_MetricDimensions {}

impl BuildDataServicequotasServiceQuota_UsageMetric_MetricDimensions {
    pub fn build(self) -> DataServicequotasServiceQuota_UsageMetric_MetricDimensions {
        DataServicequotasServiceQuota_UsageMetric_MetricDimensions {
            class: core::default::Default::default(),
            resource: core::default::Default::default(),
            service: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
        DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef {
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
pub struct DataServicequotasServiceQuota_UsageMetric {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_dimensions: std
    ::option
    ::Option<terrars::ListField<DataServicequotasServiceQuota_UsageMetric_MetricDimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_statistic_recommendation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicequotasServiceQuota_UsageMetric {
    #[doc = "Set the field `metric_dimensions`.\n"]
    pub fn set_metric_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataServicequotasServiceQuota_UsageMetric_MetricDimensions>>,
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

impl terrars::ToListMappable for DataServicequotasServiceQuota_UsageMetric {
    type O = terrars::BlockAssignable<DataServicequotasServiceQuota_UsageMetric>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicequotasServiceQuota_UsageMetric {}

impl BuildDataServicequotasServiceQuota_UsageMetric {
    pub fn build(self) -> DataServicequotasServiceQuota_UsageMetric {
        DataServicequotasServiceQuota_UsageMetric {
            metric_dimensions: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            metric_namespace: core::default::Default::default(),
            metric_statistic_recommendation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataServicequotasServiceQuota_UsageMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicequotasServiceQuota_UsageMetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataServicequotasServiceQuota_UsageMetricRef {
        DataServicequotasServiceQuota_UsageMetricRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicequotasServiceQuota_UsageMetricRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_dimensions` after provisioning.\n"]
    pub fn metric_dimensions(&self) -> terrars::ListRef<DataServicequotasServiceQuota_UsageMetric_MetricDimensionsRef> {
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
