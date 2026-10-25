#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OamLinkData_ {
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
    label_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    sink_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    link_configuration: std::option::Option<std::vec::Vec<OamLink_LinkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OamLink_Timeouts>,
    dynamic: OamLinkDynamic,
}

struct OamLink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OamLinkData_>,
}

#[derive(Clone)]
pub struct OamLink(std::rc::Rc<OamLink_>);

impl OamLink {
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

    #[doc = "Set the field `link_configuration`.\n"]
    pub fn set_link_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OamLink_LinkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().link_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.link_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OamLink_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_identifier` after provisioning.\n"]
    pub fn sink_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> terrars::ListRef<OamLink_LinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OamLink_TimeoutsRef {
        OamLink_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OamLink {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OamLink { }

impl terrars::ToListMappable for OamLink {
    type O = terrars::ListRef<OamLinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OamLink_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_oam_link".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOamLink {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub label_template: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub sink_identifier: terrars::PrimField<std::string::String>,
}

impl BuildOamLink {
    pub fn build(self, stack: &mut terrars::Stack) -> OamLink {
        let out = OamLink(std::rc::Rc::new(OamLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OamLinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                label_template: self.label_template,
                region: core::default::Default::default(),
                resource_types: self.resource_types,
                sink_identifier: self.sink_identifier,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                link_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OamLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OamLinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OamLinkRef {
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

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_identifier` after provisioning.\n"]
    pub fn sink_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> terrars::ListRef<OamLink_LinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OamLink_TimeoutsRef {
        OamLink_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OamLinkDynamic {
    link_configuration: std::option::Option<terrars::DynamicBlock<OamLink_LinkConfiguration>>,
}

#[derive(Serialize)]
pub struct OamLink_LinkConfiguration_LogGroupConfiguration {
    filter: terrars::PrimField<std::string::String>,
}

impl OamLink_LinkConfiguration_LogGroupConfiguration { }

impl terrars::ToListMappable for OamLink_LinkConfiguration_LogGroupConfiguration {
    type O = terrars::BlockAssignable<OamLink_LinkConfiguration_LogGroupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLink_LinkConfiguration_LogGroupConfiguration {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildOamLink_LinkConfiguration_LogGroupConfiguration {
    pub fn build(self) -> OamLink_LinkConfiguration_LogGroupConfiguration {
        OamLink_LinkConfiguration_LogGroupConfiguration { filter: self.filter }
    }
}

#[allow(dead_code)]
pub struct OamLink_LinkConfiguration_LogGroupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OamLink_LinkConfiguration_LogGroupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OamLink_LinkConfiguration_LogGroupConfigurationRef {
        OamLink_LinkConfiguration_LogGroupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLink_LinkConfiguration_LogGroupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct OamLink_LinkConfiguration_MetricConfiguration {
    filter: terrars::PrimField<std::string::String>,
}

impl OamLink_LinkConfiguration_MetricConfiguration { }

impl terrars::ToListMappable for OamLink_LinkConfiguration_MetricConfiguration {
    type O = terrars::BlockAssignable<OamLink_LinkConfiguration_MetricConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLink_LinkConfiguration_MetricConfiguration {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildOamLink_LinkConfiguration_MetricConfiguration {
    pub fn build(self) -> OamLink_LinkConfiguration_MetricConfiguration {
        OamLink_LinkConfiguration_MetricConfiguration { filter: self.filter }
    }
}

#[allow(dead_code)]
pub struct OamLink_LinkConfiguration_MetricConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OamLink_LinkConfiguration_MetricConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OamLink_LinkConfiguration_MetricConfigurationRef {
        OamLink_LinkConfiguration_MetricConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLink_LinkConfiguration_MetricConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct OamLink_LinkConfigurationDynamic {
    log_group_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<OamLink_LinkConfiguration_LogGroupConfiguration>>,
    metric_configuration: std::option::Option<terrars::DynamicBlock<OamLink_LinkConfiguration_MetricConfiguration>>,
}

#[derive(Serialize)]
pub struct OamLink_LinkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_configuration: std::option::Option<std::vec::Vec<OamLink_LinkConfiguration_LogGroupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_configuration: std::option::Option<std::vec::Vec<OamLink_LinkConfiguration_MetricConfiguration>>,
    dynamic: OamLink_LinkConfigurationDynamic,
}

impl OamLink_LinkConfiguration {
    #[doc = "Set the field `log_group_configuration`.\n"]
    pub fn set_log_group_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OamLink_LinkConfiguration_LogGroupConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_group_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_group_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_configuration`.\n"]
    pub fn set_metric_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OamLink_LinkConfiguration_MetricConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OamLink_LinkConfiguration {
    type O = terrars::BlockAssignable<OamLink_LinkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLink_LinkConfiguration {}

impl BuildOamLink_LinkConfiguration {
    pub fn build(self) -> OamLink_LinkConfiguration {
        OamLink_LinkConfiguration {
            log_group_configuration: core::default::Default::default(),
            metric_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OamLink_LinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OamLink_LinkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OamLink_LinkConfigurationRef {
        OamLink_LinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLink_LinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_configuration` after provisioning.\n"]
    pub fn log_group_configuration(&self) -> terrars::ListRef<OamLink_LinkConfiguration_LogGroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_group_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_configuration` after provisioning.\n"]
    pub fn metric_configuration(&self) -> terrars::ListRef<OamLink_LinkConfiguration_MetricConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct OamLink_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OamLink_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OamLink_Timeouts {
    type O = terrars::BlockAssignable<OamLink_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLink_Timeouts {}

impl BuildOamLink_Timeouts {
    pub fn build(self) -> OamLink_Timeouts {
        OamLink_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OamLink_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OamLink_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OamLink_TimeoutsRef {
        OamLink_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLink_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
