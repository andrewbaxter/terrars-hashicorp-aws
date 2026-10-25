#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PrometheusScraperLoggingConfigurationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scraper_components: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    scraper_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_destination: std::option::Option<std::vec::Vec<PrometheusScraperLoggingConfiguration_LoggingDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrometheusScraperLoggingConfiguration_Timeouts>,
    dynamic: PrometheusScraperLoggingConfigurationDynamic,
}

struct PrometheusScraperLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrometheusScraperLoggingConfigurationData_>,
}

#[derive(Clone)]
pub struct PrometheusScraperLoggingConfiguration(std::rc::Rc<PrometheusScraperLoggingConfiguration_>);

impl PrometheusScraperLoggingConfiguration {
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

    #[doc = "Set the field `scraper_components`.\n"]
    pub fn set_scraper_components(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().scraper_components = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_destination`.\n"]
    pub fn set_logging_destination(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrometheusScraperLoggingConfiguration_LoggingDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrometheusScraperLoggingConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scraper_components` after provisioning.\n"]
    pub fn scraper_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scraper_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scraper_id` after provisioning.\n"]
    pub fn scraper_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scraper_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_destination` after provisioning.\n"]
    pub fn logging_destination(&self) -> terrars::ListRef<PrometheusScraperLoggingConfiguration_LoggingDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraperLoggingConfiguration_TimeoutsRef {
        PrometheusScraperLoggingConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for PrometheusScraperLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrometheusScraperLoggingConfiguration { }

impl terrars::ToListMappable for PrometheusScraperLoggingConfiguration {
    type O = terrars::ListRef<PrometheusScraperLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrometheusScraperLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_prometheus_scraper_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrometheusScraperLoggingConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub scraper_id: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraperLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> PrometheusScraperLoggingConfiguration {
        let out = PrometheusScraperLoggingConfiguration(std::rc::Rc::new(PrometheusScraperLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrometheusScraperLoggingConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                scraper_components: core::default::Default::default(),
                scraper_id: self.scraper_id,
                logging_destination: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrometheusScraperLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraperLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrometheusScraperLoggingConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scraper_components` after provisioning.\n"]
    pub fn scraper_components(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scraper_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scraper_id` after provisioning.\n"]
    pub fn scraper_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scraper_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_destination` after provisioning.\n"]
    pub fn logging_destination(&self) -> terrars::ListRef<PrometheusScraperLoggingConfiguration_LoggingDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraperLoggingConfiguration_TimeoutsRef {
        PrometheusScraperLoggingConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraperLoggingConfigurationDynamic {
    logging_destination: std
    ::option
    ::Option<terrars::DynamicBlock<PrometheusScraperLoggingConfiguration_LoggingDestination>>,
}

#[derive(Serialize)]
pub struct PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs {
    log_group_arn: terrars::PrimField<std::string::String>,
}

impl PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs { }

impl terrars::ToListMappable for PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs {
    type O = terrars::BlockAssignable<PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs {
    #[doc = ""]
    pub log_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs {
    pub fn build(self) -> PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs {
        PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs { log_group_arn: self.log_group_arn }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef {
        PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraperLoggingConfiguration_LoggingDestinationDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs>>,
}

#[derive(Serialize)]
pub struct PrometheusScraperLoggingConfiguration_LoggingDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<std::vec::Vec<PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs>>,
    dynamic: PrometheusScraperLoggingConfiguration_LoggingDestinationDynamic,
}

impl PrometheusScraperLoggingConfiguration_LoggingDestination {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusScraperLoggingConfiguration_LoggingDestination {
    type O = terrars::BlockAssignable<PrometheusScraperLoggingConfiguration_LoggingDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraperLoggingConfiguration_LoggingDestination {}

impl BuildPrometheusScraperLoggingConfiguration_LoggingDestination {
    pub fn build(self) -> PrometheusScraperLoggingConfiguration_LoggingDestination {
        PrometheusScraperLoggingConfiguration_LoggingDestination {
            cloudwatch_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraperLoggingConfiguration_LoggingDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraperLoggingConfiguration_LoggingDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusScraperLoggingConfiguration_LoggingDestinationRef {
        PrometheusScraperLoggingConfiguration_LoggingDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraperLoggingConfiguration_LoggingDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<PrometheusScraperLoggingConfiguration_LoggingDestination_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraperLoggingConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrometheusScraperLoggingConfiguration_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusScraperLoggingConfiguration_Timeouts {
    type O = terrars::BlockAssignable<PrometheusScraperLoggingConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraperLoggingConfiguration_Timeouts {}

impl BuildPrometheusScraperLoggingConfiguration_Timeouts {
    pub fn build(self) -> PrometheusScraperLoggingConfiguration_Timeouts {
        PrometheusScraperLoggingConfiguration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraperLoggingConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraperLoggingConfiguration_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrometheusScraperLoggingConfiguration_TimeoutsRef {
        PrometheusScraperLoggingConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraperLoggingConfiguration_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
