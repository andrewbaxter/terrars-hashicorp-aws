#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PrometheusScraperData_ {
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
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    scrape_configuration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<PrometheusScraper_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exporter: std::option::Option<std::vec::Vec<PrometheusScraper_Exporter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_configuration: std::option::Option<std::vec::Vec<PrometheusScraper_RoleConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<PrometheusScraper_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrometheusScraper_Timeouts>,
    dynamic: PrometheusScraperDynamic,
}

struct PrometheusScraper_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrometheusScraperData_>,
}

#[derive(Clone)]
pub struct PrometheusScraper(std::rc::Rc<PrometheusScraper_>);

impl PrometheusScraper {
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

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exporter`.\n"]
    pub fn set_exporter(self, v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Exporter>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exporter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exporter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `role_configuration`.\n"]
    pub fn set_role_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_RoleConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().role_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.role_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Source>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrometheusScraper_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scrape_configuration` after provisioning.\n"]
    pub fn scrape_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scrape_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<PrometheusScraper_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exporter` after provisioning.\n"]
    pub fn exporter(&self) -> terrars::ListRef<PrometheusScraper_ExporterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exporter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_configuration` after provisioning.\n"]
    pub fn role_configuration(&self) -> terrars::ListRef<PrometheusScraper_RoleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.role_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<PrometheusScraper_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraper_TimeoutsRef {
        PrometheusScraper_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PrometheusScraper {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrometheusScraper { }

impl terrars::ToListMappable for PrometheusScraper {
    type O = terrars::ListRef<PrometheusScraperRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrometheusScraper_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_prometheus_scraper".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrometheusScraper {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub scrape_configuration: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraper {
    pub fn build(self, stack: &mut terrars::Stack) -> PrometheusScraper {
        let out = PrometheusScraper(std::rc::Rc::new(PrometheusScraper_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrometheusScraperData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: core::default::Default::default(),
                region: core::default::Default::default(),
                scrape_configuration: self.scrape_configuration,
                tags: core::default::Default::default(),
                destination: core::default::Default::default(),
                exporter: core::default::Default::default(),
                role_configuration: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrometheusScraperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraperRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrometheusScraperRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scrape_configuration` after provisioning.\n"]
    pub fn scrape_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scrape_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<PrometheusScraper_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exporter` after provisioning.\n"]
    pub fn exporter(&self) -> terrars::ListRef<PrometheusScraper_ExporterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exporter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_configuration` after provisioning.\n"]
    pub fn role_configuration(&self) -> terrars::ListRef<PrometheusScraper_RoleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.role_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<PrometheusScraper_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraper_TimeoutsRef {
        PrometheusScraper_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraperDynamic {
    destination: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Destination>>,
    exporter: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Exporter>>,
    role_configuration: std::option::Option<terrars::DynamicBlock<PrometheusScraper_RoleConfiguration>>,
    source: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Source>>,
}

#[derive(Serialize)]
pub struct PrometheusScraper_Destination_Amp {
    workspace_arn: terrars::PrimField<std::string::String>,
}

impl PrometheusScraper_Destination_Amp { }

impl terrars::ToListMappable for PrometheusScraper_Destination_Amp {
    type O = terrars::BlockAssignable<PrometheusScraper_Destination_Amp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Destination_Amp {
    #[doc = ""]
    pub workspace_arn: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraper_Destination_Amp {
    pub fn build(self) -> PrometheusScraper_Destination_Amp {
        PrometheusScraper_Destination_Amp { workspace_arn: self.workspace_arn }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_Destination_AmpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_Destination_AmpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_Destination_AmpRef {
        PrometheusScraper_Destination_AmpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_Destination_AmpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_arn` after provisioning.\n"]
    pub fn workspace_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_Destination_Cloudwatch {
    dataset_arn: terrars::PrimField<std::string::String>,
}

impl PrometheusScraper_Destination_Cloudwatch { }

impl terrars::ToListMappable for PrometheusScraper_Destination_Cloudwatch {
    type O = terrars::BlockAssignable<PrometheusScraper_Destination_Cloudwatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Destination_Cloudwatch {
    #[doc = ""]
    pub dataset_arn: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraper_Destination_Cloudwatch {
    pub fn build(self) -> PrometheusScraper_Destination_Cloudwatch {
        PrometheusScraper_Destination_Cloudwatch { dataset_arn: self.dataset_arn }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_Destination_CloudwatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_Destination_CloudwatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_Destination_CloudwatchRef {
        PrometheusScraper_Destination_CloudwatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_Destination_CloudwatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_arn` after provisioning.\n"]
    pub fn dataset_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraper_DestinationDynamic {
    amp: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Destination_Amp>>,
    cloudwatch: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Destination_Cloudwatch>>,
}

#[derive(Serialize)]
pub struct PrometheusScraper_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amp: std::option::Option<std::vec::Vec<PrometheusScraper_Destination_Amp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch: std::option::Option<std::vec::Vec<PrometheusScraper_Destination_Cloudwatch>>,
    dynamic: PrometheusScraper_DestinationDynamic,
}

impl PrometheusScraper_Destination {
    #[doc = "Set the field `amp`.\n"]
    pub fn set_amp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Destination_Amp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amp = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch`.\n"]
    pub fn set_cloudwatch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Destination_Cloudwatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusScraper_Destination {
    type O = terrars::BlockAssignable<PrometheusScraper_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Destination {}

impl BuildPrometheusScraper_Destination {
    pub fn build(self) -> PrometheusScraper_Destination {
        PrometheusScraper_Destination {
            amp: core::default::Default::default(),
            cloudwatch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_DestinationRef {
        PrometheusScraper_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amp` after provisioning.\n"]
    pub fn amp(&self) -> terrars::ListRef<PrometheusScraper_Destination_AmpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amp", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch` after provisioning.\n"]
    pub fn cloudwatch(&self) -> terrars::ListRef<PrometheusScraper_Destination_CloudwatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_Exporter_Opensearch {
    domain_arn: terrars::PrimField<std::string::String>,
}

impl PrometheusScraper_Exporter_Opensearch { }

impl terrars::ToListMappable for PrometheusScraper_Exporter_Opensearch {
    type O = terrars::BlockAssignable<PrometheusScraper_Exporter_Opensearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Exporter_Opensearch {
    #[doc = ""]
    pub domain_arn: terrars::PrimField<std::string::String>,
}

impl BuildPrometheusScraper_Exporter_Opensearch {
    pub fn build(self) -> PrometheusScraper_Exporter_Opensearch {
        PrometheusScraper_Exporter_Opensearch { domain_arn: self.domain_arn }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_Exporter_OpensearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_Exporter_OpensearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_Exporter_OpensearchRef {
        PrometheusScraper_Exporter_OpensearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_Exporter_OpensearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_arn` after provisioning.\n"]
    pub fn domain_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraper_ExporterDynamic {
    opensearch: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Exporter_Opensearch>>,
}

#[derive(Serialize)]
pub struct PrometheusScraper_Exporter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearch: std::option::Option<std::vec::Vec<PrometheusScraper_Exporter_Opensearch>>,
    dynamic: PrometheusScraper_ExporterDynamic,
}

impl PrometheusScraper_Exporter {
    #[doc = "Set the field `opensearch`.\n"]
    pub fn set_opensearch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Exporter_Opensearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.opensearch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.opensearch = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusScraper_Exporter {
    type O = terrars::BlockAssignable<PrometheusScraper_Exporter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Exporter {}

impl BuildPrometheusScraper_Exporter {
    pub fn build(self) -> PrometheusScraper_Exporter {
        PrometheusScraper_Exporter {
            opensearch: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_ExporterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_ExporterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_ExporterRef {
        PrometheusScraper_ExporterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_ExporterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opensearch` after provisioning.\n"]
    pub fn opensearch(&self) -> terrars::ListRef<PrometheusScraper_Exporter_OpensearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearch", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_RoleConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrometheusScraper_RoleConfiguration {
    #[doc = "Set the field `source_role_arn`.\n"]
    pub fn set_source_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_role_arn`.\n"]
    pub fn set_target_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusScraper_RoleConfiguration {
    type O = terrars::BlockAssignable<PrometheusScraper_RoleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_RoleConfiguration {}

impl BuildPrometheusScraper_RoleConfiguration {
    pub fn build(self) -> PrometheusScraper_RoleConfiguration {
        PrometheusScraper_RoleConfiguration {
            source_role_arn: core::default::Default::default(),
            target_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_RoleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_RoleConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_RoleConfigurationRef {
        PrometheusScraper_RoleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_RoleConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_role_arn` after provisioning.\n"]
    pub fn source_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_role_arn` after provisioning.\n"]
    pub fn target_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_Source_Eks {
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl PrometheusScraper_Source_Eks {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrometheusScraper_Source_Eks {
    type O = terrars::BlockAssignable<PrometheusScraper_Source_Eks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Source_Eks {
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPrometheusScraper_Source_Eks {
    pub fn build(self) -> PrometheusScraper_Source_Eks {
        PrometheusScraper_Source_Eks {
            cluster_arn: self.cluster_arn,
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_Source_EksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_Source_EksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_Source_EksRef {
        PrometheusScraper_Source_EksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_Source_EksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_Source_Vpc {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl PrometheusScraper_Source_Vpc { }

impl terrars::ToListMappable for PrometheusScraper_Source_Vpc {
    type O = terrars::BlockAssignable<PrometheusScraper_Source_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Source_Vpc {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPrometheusScraper_Source_Vpc {
    pub fn build(self) -> PrometheusScraper_Source_Vpc {
        PrometheusScraper_Source_Vpc {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_Source_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_Source_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_Source_VpcRef {
        PrometheusScraper_Source_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_Source_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusScraper_SourceDynamic {
    eks: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Source_Eks>>,
    vpc: std::option::Option<terrars::DynamicBlock<PrometheusScraper_Source_Vpc>>,
}

#[derive(Serialize)]
pub struct PrometheusScraper_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks: std::option::Option<std::vec::Vec<PrometheusScraper_Source_Eks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<PrometheusScraper_Source_Vpc>>,
    dynamic: PrometheusScraper_SourceDynamic,
}

impl PrometheusScraper_Source {
    #[doc = "Set the field `eks`.\n"]
    pub fn set_eks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Source_Eks>>,
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

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrometheusScraper_Source_Vpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrometheusScraper_Source {
    type O = terrars::BlockAssignable<PrometheusScraper_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Source {}

impl BuildPrometheusScraper_Source {
    pub fn build(self) -> PrometheusScraper_Source {
        PrometheusScraper_Source {
            eks: core::default::Default::default(),
            vpc: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_SourceRef {
        PrometheusScraper_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks` after provisioning.\n"]
    pub fn eks(&self) -> terrars::ListRef<PrometheusScraper_Source_EksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<PrometheusScraper_Source_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusScraper_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrometheusScraper_Timeouts {
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

impl terrars::ToListMappable for PrometheusScraper_Timeouts {
    type O = terrars::BlockAssignable<PrometheusScraper_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusScraper_Timeouts {}

impl BuildPrometheusScraper_Timeouts {
    pub fn build(self) -> PrometheusScraper_Timeouts {
        PrometheusScraper_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrometheusScraper_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrometheusScraper_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrometheusScraper_TimeoutsRef {
        PrometheusScraper_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusScraper_TimeoutsRef {
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
