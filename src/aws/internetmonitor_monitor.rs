#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct InternetmonitorMonitorData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_city_networks_to_monitor: std::option::Option<terrars::PrimField<f64>>,
    monitor_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_percentage_to_monitor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_events_config: std::option::Option<std::vec::Vec<InternetmonitorMonitor_HealthEventsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_measurements_log_delivery: std
    ::option
    ::Option<std::vec::Vec<InternetmonitorMonitor_InternetMeasurementsLogDelivery>>,
    dynamic: InternetmonitorMonitorDynamic,
}

struct InternetmonitorMonitor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<InternetmonitorMonitorData_>,
}

#[derive(Clone)]
pub struct InternetmonitorMonitor(std::rc::Rc<InternetmonitorMonitor_>);

impl InternetmonitorMonitor {
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

    #[doc = "Set the field `max_city_networks_to_monitor`.\n"]
    pub fn set_max_city_networks_to_monitor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_city_networks_to_monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `traffic_percentage_to_monitor`.\n"]
    pub fn set_traffic_percentage_to_monitor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().traffic_percentage_to_monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_events_config`.\n"]
    pub fn set_health_events_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<InternetmonitorMonitor_HealthEventsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_events_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_events_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `internet_measurements_log_delivery`.\n"]
    pub fn set_internet_measurements_log_delivery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<InternetmonitorMonitor_InternetMeasurementsLogDelivery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().internet_measurements_log_delivery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.internet_measurements_log_delivery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_city_networks_to_monitor` after provisioning.\n"]
    pub fn max_city_networks_to_monitor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_city_networks_to_monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_percentage_to_monitor` after provisioning.\n"]
    pub fn traffic_percentage_to_monitor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_percentage_to_monitor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_events_config` after provisioning.\n"]
    pub fn health_events_config(&self) -> terrars::ListRef<InternetmonitorMonitor_HealthEventsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_events_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internet_measurements_log_delivery` after provisioning.\n"]
    pub fn internet_measurements_log_delivery(
        &self,
    ) -> terrars::ListRef<InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.internet_measurements_log_delivery", self.extract_ref()),
        )
    }
}

impl terrars::Referable for InternetmonitorMonitor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for InternetmonitorMonitor { }

impl terrars::ToListMappable for InternetmonitorMonitor {
    type O = terrars::ListRef<InternetmonitorMonitorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for InternetmonitorMonitor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_internetmonitor_monitor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInternetmonitorMonitor {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub monitor_name: terrars::PrimField<std::string::String>,
}

impl BuildInternetmonitorMonitor {
    pub fn build(self, stack: &mut terrars::Stack) -> InternetmonitorMonitor {
        let out = InternetmonitorMonitor(std::rc::Rc::new(InternetmonitorMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(InternetmonitorMonitorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                max_city_networks_to_monitor: core::default::Default::default(),
                monitor_name: self.monitor_name,
                region: core::default::Default::default(),
                resources: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_percentage_to_monitor: core::default::Default::default(),
                health_events_config: core::default::Default::default(),
                internet_measurements_log_delivery: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct InternetmonitorMonitorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InternetmonitorMonitorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl InternetmonitorMonitorRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_city_networks_to_monitor` after provisioning.\n"]
    pub fn max_city_networks_to_monitor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_city_networks_to_monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_percentage_to_monitor` after provisioning.\n"]
    pub fn traffic_percentage_to_monitor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_percentage_to_monitor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_events_config` after provisioning.\n"]
    pub fn health_events_config(&self) -> terrars::ListRef<InternetmonitorMonitor_HealthEventsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_events_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internet_measurements_log_delivery` after provisioning.\n"]
    pub fn internet_measurements_log_delivery(
        &self,
    ) -> terrars::ListRef<InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.internet_measurements_log_delivery", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct InternetmonitorMonitorDynamic {
    health_events_config: std::option::Option<terrars::DynamicBlock<InternetmonitorMonitor_HealthEventsConfig>>,
    internet_measurements_log_delivery: std
    ::option
    ::Option<terrars::DynamicBlock<InternetmonitorMonitor_InternetMeasurementsLogDelivery>>,
}

#[derive(Serialize)]
pub struct InternetmonitorMonitor_HealthEventsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_score_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_score_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl InternetmonitorMonitor_HealthEventsConfig {
    #[doc = "Set the field `availability_score_threshold`.\n"]
    pub fn set_availability_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_score_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_score_threshold`.\n"]
    pub fn set_performance_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.performance_score_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for InternetmonitorMonitor_HealthEventsConfig {
    type O = terrars::BlockAssignable<InternetmonitorMonitor_HealthEventsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInternetmonitorMonitor_HealthEventsConfig {}

impl BuildInternetmonitorMonitor_HealthEventsConfig {
    pub fn build(self) -> InternetmonitorMonitor_HealthEventsConfig {
        InternetmonitorMonitor_HealthEventsConfig {
            availability_score_threshold: core::default::Default::default(),
            performance_score_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct InternetmonitorMonitor_HealthEventsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InternetmonitorMonitor_HealthEventsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> InternetmonitorMonitor_HealthEventsConfigRef {
        InternetmonitorMonitor_HealthEventsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InternetmonitorMonitor_HealthEventsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_score_threshold` after provisioning.\n"]
    pub fn availability_score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_score_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `performance_score_threshold` after provisioning.\n"]
    pub fn performance_score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_score_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_delivery_status`.\n"]
    pub fn set_log_delivery_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_delivery_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
    type O = terrars::BlockAssignable<InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildInternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
    pub fn build(self) -> InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
        InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            log_delivery_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef {
        InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `log_delivery_status` after provisioning.\n"]
    pub fn log_delivery_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_delivery_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct InternetmonitorMonitor_InternetMeasurementsLogDeliveryDynamic {
    s3_config: std
    ::option
    ::Option<terrars::DynamicBlock<InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config>>,
}

#[derive(Serialize)]
pub struct InternetmonitorMonitor_InternetMeasurementsLogDelivery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_config: std::option::Option<std::vec::Vec<InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config>>,
    dynamic: InternetmonitorMonitor_InternetMeasurementsLogDeliveryDynamic,
}

impl InternetmonitorMonitor_InternetMeasurementsLogDelivery {
    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for InternetmonitorMonitor_InternetMeasurementsLogDelivery {
    type O = terrars::BlockAssignable<InternetmonitorMonitor_InternetMeasurementsLogDelivery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInternetmonitorMonitor_InternetMeasurementsLogDelivery {}

impl BuildInternetmonitorMonitor_InternetMeasurementsLogDelivery {
    pub fn build(self) -> InternetmonitorMonitor_InternetMeasurementsLogDelivery {
        InternetmonitorMonitor_InternetMeasurementsLogDelivery {
            s3_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef {
        InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InternetmonitorMonitor_InternetMeasurementsLogDeliveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> terrars::ListRef<InternetmonitorMonitor_InternetMeasurementsLogDelivery_S3ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}
