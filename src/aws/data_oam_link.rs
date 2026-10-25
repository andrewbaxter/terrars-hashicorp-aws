#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOamLinkData_ {
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
    link_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataOamLink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOamLinkData_>,
}

#[derive(Clone)]
pub struct DataOamLink(std::rc::Rc<DataOamLink_>);

impl DataOamLink {
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

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> terrars::ListRef<DataOamLink_LinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_identifier` after provisioning.\n"]
    pub fn link_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataOamLink {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOamLink { }

impl terrars::ToListMappable for DataOamLink {
    type O = terrars::ListRef<DataOamLinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOamLink_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_oam_link".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOamLink {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub link_identifier: terrars::PrimField<std::string::String>,
}

impl BuildDataOamLink {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOamLink {
        let out = DataOamLink(std::rc::Rc::new(DataOamLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOamLinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                link_identifier: self.link_identifier,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOamLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOamLinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOamLinkRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> terrars::ListRef<DataOamLink_LinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_identifier` after provisioning.\n"]
    pub fn link_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOamLink_LinkConfiguration_LogGroupConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOamLink_LinkConfiguration_LogGroupConfiguration {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOamLink_LinkConfiguration_LogGroupConfiguration {
    type O = terrars::BlockAssignable<DataOamLink_LinkConfiguration_LogGroupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLink_LinkConfiguration_LogGroupConfiguration {}

impl BuildDataOamLink_LinkConfiguration_LogGroupConfiguration {
    pub fn build(self) -> DataOamLink_LinkConfiguration_LogGroupConfiguration {
        DataOamLink_LinkConfiguration_LogGroupConfiguration { filter: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOamLink_LinkConfiguration_LogGroupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOamLink_LinkConfiguration_LogGroupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOamLink_LinkConfiguration_LogGroupConfigurationRef {
        DataOamLink_LinkConfiguration_LogGroupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLink_LinkConfiguration_LogGroupConfigurationRef {
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
pub struct DataOamLink_LinkConfiguration_MetricConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOamLink_LinkConfiguration_MetricConfiguration {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOamLink_LinkConfiguration_MetricConfiguration {
    type O = terrars::BlockAssignable<DataOamLink_LinkConfiguration_MetricConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLink_LinkConfiguration_MetricConfiguration {}

impl BuildDataOamLink_LinkConfiguration_MetricConfiguration {
    pub fn build(self) -> DataOamLink_LinkConfiguration_MetricConfiguration {
        DataOamLink_LinkConfiguration_MetricConfiguration { filter: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOamLink_LinkConfiguration_MetricConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOamLink_LinkConfiguration_MetricConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOamLink_LinkConfiguration_MetricConfigurationRef {
        DataOamLink_LinkConfiguration_MetricConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLink_LinkConfiguration_MetricConfigurationRef {
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
pub struct DataOamLink_LinkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_configuration: std::option::Option<terrars::ListField<DataOamLink_LinkConfiguration_LogGroupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_configuration: std::option::Option<terrars::ListField<DataOamLink_LinkConfiguration_MetricConfiguration>>,
}

impl DataOamLink_LinkConfiguration {
    #[doc = "Set the field `log_group_configuration`.\n"]
    pub fn set_log_group_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOamLink_LinkConfiguration_LogGroupConfiguration>>,
    ) -> Self {
        self.log_group_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_configuration`.\n"]
    pub fn set_metric_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOamLink_LinkConfiguration_MetricConfiguration>>,
    ) -> Self {
        self.metric_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOamLink_LinkConfiguration {
    type O = terrars::BlockAssignable<DataOamLink_LinkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLink_LinkConfiguration {}

impl BuildDataOamLink_LinkConfiguration {
    pub fn build(self) -> DataOamLink_LinkConfiguration {
        DataOamLink_LinkConfiguration {
            log_group_configuration: core::default::Default::default(),
            metric_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOamLink_LinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOamLink_LinkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOamLink_LinkConfigurationRef {
        DataOamLink_LinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLink_LinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_configuration` after provisioning.\n"]
    pub fn log_group_configuration(&self) -> terrars::ListRef<DataOamLink_LinkConfiguration_LogGroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_group_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_configuration` after provisioning.\n"]
    pub fn metric_configuration(&self) -> terrars::ListRef<DataOamLink_LinkConfiguration_MetricConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_configuration", self.base))
    }
}
