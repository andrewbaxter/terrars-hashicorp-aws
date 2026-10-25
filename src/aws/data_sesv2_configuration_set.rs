#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSesv2ConfigurationSetData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    configuration_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataSesv2ConfigurationSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSesv2ConfigurationSetData_>,
}

#[derive(Clone)]
pub struct DataSesv2ConfigurationSet(std::rc::Rc<DataSesv2ConfigurationSet_>);

impl DataSesv2ConfigurationSet {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_DeliveryOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_ReputationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reputation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_SendingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sending_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_SuppressionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppression_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_TrackingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_VdmOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vdm_options", self.extract_ref()))
    }
}

impl terrars::Referable for DataSesv2ConfigurationSet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSesv2ConfigurationSet { }

impl terrars::ToListMappable for DataSesv2ConfigurationSet {
    type O = terrars::ListRef<DataSesv2ConfigurationSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSesv2ConfigurationSet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_sesv2_configuration_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2ConfigurationSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub configuration_set_name: terrars::PrimField<std::string::String>,
}

impl BuildDataSesv2ConfigurationSet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSesv2ConfigurationSet {
        let out = DataSesv2ConfigurationSet(std::rc::Rc::new(DataSesv2ConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSesv2ConfigurationSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                configuration_set_name: self.configuration_set_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2ConfigurationSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSesv2ConfigurationSetRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_DeliveryOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_ReputationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reputation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_SendingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sending_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_SuppressionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppression_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_TrackingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vdm_options` after provisioning.\n"]
    pub fn vdm_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_VdmOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vdm_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_DeliveryOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_delivery_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_pool_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSesv2ConfigurationSet_DeliveryOptions {
    #[doc = "Set the field `max_delivery_seconds`.\n"]
    pub fn set_max_delivery_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_delivery_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sending_pool_name`.\n"]
    pub fn set_sending_pool_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sending_pool_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_DeliveryOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_DeliveryOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_DeliveryOptions {}

impl BuildDataSesv2ConfigurationSet_DeliveryOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_DeliveryOptions {
        DataSesv2ConfigurationSet_DeliveryOptions {
            max_delivery_seconds: core::default::Default::default(),
            sending_pool_name: core::default::Default::default(),
            tls_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_DeliveryOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_DeliveryOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSesv2ConfigurationSet_DeliveryOptionsRef {
        DataSesv2ConfigurationSet_DeliveryOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_DeliveryOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_delivery_seconds` after provisioning.\n"]
    pub fn max_delivery_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_delivery_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `sending_pool_name` after provisioning.\n"]
    pub fn sending_pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sending_pool_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_ReputationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_fresh_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reputation_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSesv2ConfigurationSet_ReputationOptions {
    #[doc = "Set the field `last_fresh_start`.\n"]
    pub fn set_last_fresh_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_fresh_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reputation_metrics_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_ReputationOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_ReputationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_ReputationOptions {}

impl BuildDataSesv2ConfigurationSet_ReputationOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_ReputationOptions {
        DataSesv2ConfigurationSet_ReputationOptions {
            last_fresh_start: core::default::Default::default(),
            reputation_metrics_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_ReputationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_ReputationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSesv2ConfigurationSet_ReputationOptionsRef {
        DataSesv2ConfigurationSet_ReputationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_ReputationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_fresh_start", self.base))
    }

    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reputation_metrics_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_SendingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataSesv2ConfigurationSet_SendingOptions {
    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sending_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_SendingOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_SendingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_SendingOptions {}

impl BuildDataSesv2ConfigurationSet_SendingOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_SendingOptions {
        DataSesv2ConfigurationSet_SendingOptions { sending_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_SendingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_SendingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSesv2ConfigurationSet_SendingOptionsRef {
        DataSesv2ConfigurationSet_SendingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_SendingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sending_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_SuppressionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppressed_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSesv2ConfigurationSet_SuppressionOptions {
    #[doc = "Set the field `suppressed_reasons`.\n"]
    pub fn set_suppressed_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.suppressed_reasons = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_SuppressionOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_SuppressionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_SuppressionOptions {}

impl BuildDataSesv2ConfigurationSet_SuppressionOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_SuppressionOptions {
        DataSesv2ConfigurationSet_SuppressionOptions { suppressed_reasons: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_SuppressionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_SuppressionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSesv2ConfigurationSet_SuppressionOptionsRef {
        DataSesv2ConfigurationSet_SuppressionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_SuppressionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `suppressed_reasons` after provisioning.\n"]
    pub fn suppressed_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppressed_reasons", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_TrackingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_redirect_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSesv2ConfigurationSet_TrackingOptions {
    #[doc = "Set the field `custom_redirect_domain`.\n"]
    pub fn set_custom_redirect_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_redirect_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `https_policy`.\n"]
    pub fn set_https_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.https_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_TrackingOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_TrackingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_TrackingOptions {}

impl BuildDataSesv2ConfigurationSet_TrackingOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_TrackingOptions {
        DataSesv2ConfigurationSet_TrackingOptions {
            custom_redirect_domain: core::default::Default::default(),
            https_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_TrackingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_TrackingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSesv2ConfigurationSet_TrackingOptionsRef {
        DataSesv2ConfigurationSet_TrackingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_TrackingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_redirect_domain` after provisioning.\n"]
    pub fn custom_redirect_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_redirect_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `https_policy` after provisioning.\n"]
    pub fn https_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engagement_metrics: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
    #[doc = "Set the field `engagement_metrics`.\n"]
    pub fn set_engagement_metrics(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.engagement_metrics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_VdmOptions_DashboardOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_VdmOptions_DashboardOptions {}

impl BuildDataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
        DataSesv2ConfigurationSet_VdmOptions_DashboardOptions {
            engagement_metrics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef {
        DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engagement_metrics` after provisioning.\n"]
    pub fn engagement_metrics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engagement_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimized_shared_delivery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
    #[doc = "Set the field `optimized_shared_delivery`.\n"]
    pub fn set_optimized_shared_delivery(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.optimized_shared_delivery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_VdmOptions_GuardianOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_VdmOptions_GuardianOptions {}

impl BuildDataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
        DataSesv2ConfigurationSet_VdmOptions_GuardianOptions {
            optimized_shared_delivery: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef {
        DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `optimized_shared_delivery` after provisioning.\n"]
    pub fn optimized_shared_delivery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimized_shared_delivery", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSesv2ConfigurationSet_VdmOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_options: std::option::Option<terrars::ListField<DataSesv2ConfigurationSet_VdmOptions_DashboardOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardian_options: std::option::Option<terrars::ListField<DataSesv2ConfigurationSet_VdmOptions_GuardianOptions>>,
}

impl DataSesv2ConfigurationSet_VdmOptions {
    #[doc = "Set the field `dashboard_options`.\n"]
    pub fn set_dashboard_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSesv2ConfigurationSet_VdmOptions_DashboardOptions>>,
    ) -> Self {
        self.dashboard_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardian_options`.\n"]
    pub fn set_guardian_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSesv2ConfigurationSet_VdmOptions_GuardianOptions>>,
    ) -> Self {
        self.guardian_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2ConfigurationSet_VdmOptions {
    type O = terrars::BlockAssignable<DataSesv2ConfigurationSet_VdmOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2ConfigurationSet_VdmOptions {}

impl BuildDataSesv2ConfigurationSet_VdmOptions {
    pub fn build(self) -> DataSesv2ConfigurationSet_VdmOptions {
        DataSesv2ConfigurationSet_VdmOptions {
            dashboard_options: core::default::Default::default(),
            guardian_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSesv2ConfigurationSet_VdmOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2ConfigurationSet_VdmOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSesv2ConfigurationSet_VdmOptionsRef {
        DataSesv2ConfigurationSet_VdmOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2ConfigurationSet_VdmOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dashboard_options` after provisioning.\n"]
    pub fn dashboard_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_VdmOptions_DashboardOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_options", self.base))
    }

    #[doc = "Get a reference to the value of field `guardian_options` after provisioning.\n"]
    pub fn guardian_options(&self) -> terrars::ListRef<DataSesv2ConfigurationSet_VdmOptions_GuardianOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardian_options", self.base))
    }
}
