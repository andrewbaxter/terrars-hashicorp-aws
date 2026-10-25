#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontRealtimeLogConfigData_ {
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
    name: terrars::PrimField<std::string::String>,
}

struct DataCloudfrontRealtimeLogConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontRealtimeLogConfigData_>,
}

#[derive(Clone)]
pub struct DataCloudfrontRealtimeLogConfig(std::rc::Rc<DataCloudfrontRealtimeLogConfig_>);

impl DataCloudfrontRealtimeLogConfig {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<DataCloudfrontRealtimeLogConfig_EndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_rate", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfrontRealtimeLogConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontRealtimeLogConfig { }

impl terrars::ToListMappable for DataCloudfrontRealtimeLogConfig {
    type O = terrars::ListRef<DataCloudfrontRealtimeLogConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontRealtimeLogConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_realtime_log_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudfrontRealtimeLogConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontRealtimeLogConfig {
        let out = DataCloudfrontRealtimeLogConfig(std::rc::Rc::new(DataCloudfrontRealtimeLogConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontRealtimeLogConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontRealtimeLogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontRealtimeLogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontRealtimeLogConfigRef {
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

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<DataCloudfrontRealtimeLogConfig_EndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_rate", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
    type O = terrars::BlockAssignable<DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {}

impl BuildDataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
    pub fn build(self) -> DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
        DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig {
            role_arn: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef {
        DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontRealtimeLogConfig_Endpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_config: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontRealtimeLogConfig_Endpoint {
    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfig>>,
    ) -> Self {
        self.kinesis_stream_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_type`.\n"]
    pub fn set_stream_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontRealtimeLogConfig_Endpoint {
    type O = terrars::BlockAssignable<DataCloudfrontRealtimeLogConfig_Endpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfig_Endpoint {}

impl BuildDataCloudfrontRealtimeLogConfig_Endpoint {
    pub fn build(self) -> DataCloudfrontRealtimeLogConfig_Endpoint {
        DataCloudfrontRealtimeLogConfig_Endpoint {
            kinesis_stream_config: core::default::Default::default(),
            stream_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontRealtimeLogConfig_EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontRealtimeLogConfig_EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfrontRealtimeLogConfig_EndpointRef {
        DataCloudfrontRealtimeLogConfig_EndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontRealtimeLogConfig_EndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontRealtimeLogConfig_Endpoint_KinesisStreamConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_type` after provisioning.\n"]
    pub fn stream_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_type", self.base))
    }
}
