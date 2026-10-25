#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectRoutingProfileData_ {
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
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataConnectRoutingProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectRoutingProfileData_>,
}

#[derive(Clone)]
pub struct DataConnectRoutingProfile(std::rc::Rc<DataConnectRoutingProfile_>);

impl DataConnectRoutingProfile {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_profile_id`.\n"]
    pub fn set_routing_profile_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().routing_profile_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `default_outbound_queue_id` after provisioning.\n"]
    pub fn default_outbound_queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_outbound_queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_concurrencies` after provisioning.\n"]
    pub fn media_concurrencies(&self) -> terrars::SetRef<DataConnectRoutingProfile_MediaConcurrenciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.media_concurrencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_configs` after provisioning.\n"]
    pub fn queue_configs(&self) -> terrars::SetRef<DataConnectRoutingProfile_QueueConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queue_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectRoutingProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectRoutingProfile { }

impl terrars::ToListMappable for DataConnectRoutingProfile {
    type O = terrars::ListRef<DataConnectRoutingProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectRoutingProfile_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_routing_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectRoutingProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectRoutingProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectRoutingProfile {
        let out = DataConnectRoutingProfile(std::rc::Rc::new(DataConnectRoutingProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectRoutingProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_profile_id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectRoutingProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectRoutingProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectRoutingProfileRef {
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

    #[doc = "Get a reference to the value of field `default_outbound_queue_id` after provisioning.\n"]
    pub fn default_outbound_queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_outbound_queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_concurrencies` after provisioning.\n"]
    pub fn media_concurrencies(&self) -> terrars::SetRef<DataConnectRoutingProfile_MediaConcurrenciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.media_concurrencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_configs` after provisioning.\n"]
    pub fn queue_configs(&self) -> terrars::SetRef<DataConnectRoutingProfile_QueueConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queue_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    behavior_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
    #[doc = "Set the field `behavior_type`.\n"]
    pub fn set_behavior_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.behavior_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
    type O = terrars::BlockAssignable<DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {}

impl BuildDataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
    pub fn build(self) -> DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
        DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior {
            behavior_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef {
        DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior_type` after provisioning.\n"]
    pub fn behavior_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectRoutingProfile_MediaConcurrencies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_channel_behavior: std
    ::option
    ::Option<terrars::ListField<DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior>>,
}

impl DataConnectRoutingProfile_MediaConcurrencies {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `concurrency`.\n"]
    pub fn set_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cross_channel_behavior`.\n"]
    pub fn set_cross_channel_behavior(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehavior>>,
    ) -> Self {
        self.cross_channel_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectRoutingProfile_MediaConcurrencies {
    type O = terrars::BlockAssignable<DataConnectRoutingProfile_MediaConcurrencies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectRoutingProfile_MediaConcurrencies {}

impl BuildDataConnectRoutingProfile_MediaConcurrencies {
    pub fn build(self) -> DataConnectRoutingProfile_MediaConcurrencies {
        DataConnectRoutingProfile_MediaConcurrencies {
            channel: core::default::Default::default(),
            concurrency: core::default::Default::default(),
            cross_channel_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectRoutingProfile_MediaConcurrenciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectRoutingProfile_MediaConcurrenciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectRoutingProfile_MediaConcurrenciesRef {
        DataConnectRoutingProfile_MediaConcurrenciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectRoutingProfile_MediaConcurrenciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\n"]
    pub fn concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_channel_behavior` after provisioning.\n"]
    pub fn cross_channel_behavior(
        &self,
    ) -> terrars::ListRef<DataConnectRoutingProfile_MediaConcurrencies_CrossChannelBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_channel_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectRoutingProfile_QueueConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectRoutingProfile_QueueConfigs {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_arn`.\n"]
    pub fn set_queue_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_id`.\n"]
    pub fn set_queue_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_name`.\n"]
    pub fn set_queue_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectRoutingProfile_QueueConfigs {
    type O = terrars::BlockAssignable<DataConnectRoutingProfile_QueueConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectRoutingProfile_QueueConfigs {}

impl BuildDataConnectRoutingProfile_QueueConfigs {
    pub fn build(self) -> DataConnectRoutingProfile_QueueConfigs {
        DataConnectRoutingProfile_QueueConfigs {
            channel: core::default::Default::default(),
            delay: core::default::Default::default(),
            priority: core::default::Default::default(),
            queue_arn: core::default::Default::default(),
            queue_id: core::default::Default::default(),
            queue_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectRoutingProfile_QueueConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectRoutingProfile_QueueConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectRoutingProfile_QueueConfigsRef {
        DataConnectRoutingProfile_QueueConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectRoutingProfile_QueueConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_arn` after provisioning.\n"]
    pub fn queue_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }
}
