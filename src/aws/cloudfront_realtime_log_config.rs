use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontRealtimeLogConfigData {
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
    fields: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sampling_rate: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<std::vec::Vec<CloudfrontRealtimeLogConfigEndpointEl>>,
    dynamic: CloudfrontRealtimeLogConfigDynamic,
}

struct CloudfrontRealtimeLogConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontRealtimeLogConfigData>,
}

#[derive(Clone)]
pub struct CloudfrontRealtimeLogConfig(std::rc::Rc<CloudfrontRealtimeLogConfig_>);

impl CloudfrontRealtimeLogConfig {
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

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontRealtimeLogConfigEndpointEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<CloudfrontRealtimeLogConfigEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontRealtimeLogConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontRealtimeLogConfig { }

impl terrars::ToListMappable for CloudfrontRealtimeLogConfig {
    type O = terrars::ListRef<CloudfrontRealtimeLogConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontRealtimeLogConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_realtime_log_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontRealtimeLogConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub fields: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sampling_rate: terrars::PrimField<f64>,
}

impl BuildCloudfrontRealtimeLogConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontRealtimeLogConfig {
        let out = CloudfrontRealtimeLogConfig(std::rc::Rc::new(CloudfrontRealtimeLogConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontRealtimeLogConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                fields: self.fields,
                id: core::default::Default::default(),
                name: self.name,
                sampling_rate: self.sampling_rate,
                endpoint: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontRealtimeLogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontRealtimeLogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontRealtimeLogConfigRef {
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

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<CloudfrontRealtimeLogConfigEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    role_arn: terrars::PrimField<std::string::String>,
    stream_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl { }

impl terrars::ToListMappable for CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    type O = terrars::BlockAssignable<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    pub fn build(self) -> CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
        CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
            role_arn: self.role_arn,
            stream_arn: self.stream_arn,
        }
    }
}

pub struct CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
        CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
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

#[derive(Serialize, Default)]
struct CloudfrontRealtimeLogConfigEndpointElDynamic {
    kinesis_stream_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontRealtimeLogConfigEndpointEl {
    stream_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_config: std::option::Option<std::vec::Vec<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    dynamic: CloudfrontRealtimeLogConfigEndpointElDynamic,
}

impl CloudfrontRealtimeLogConfigEndpointEl {
    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontRealtimeLogConfigEndpointEl {
    type O = terrars::BlockAssignable<CloudfrontRealtimeLogConfigEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontRealtimeLogConfigEndpointEl {
    #[doc = ""]
    pub stream_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontRealtimeLogConfigEndpointEl {
    pub fn build(self) -> CloudfrontRealtimeLogConfigEndpointEl {
        CloudfrontRealtimeLogConfigEndpointEl {
            stream_type: self.stream_type,
            kinesis_stream_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontRealtimeLogConfigEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontRealtimeLogConfigEndpointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontRealtimeLogConfigEndpointElRef {
        CloudfrontRealtimeLogConfigEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontRealtimeLogConfigEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_type` after provisioning.\n"]
    pub fn stream_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> terrars::ListRef<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontRealtimeLogConfigDynamic {
    endpoint: std::option::Option<terrars::DynamicBlock<CloudfrontRealtimeLogConfigEndpointEl>>,
}
