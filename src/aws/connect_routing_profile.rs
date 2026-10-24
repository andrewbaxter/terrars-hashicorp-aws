use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectRoutingProfileData {
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
    default_outbound_queue_id: terrars::PrimField<std::string::String>,
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_concurrencies: std::option::Option<std::vec::Vec<ConnectRoutingProfileMediaConcurrenciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_configs: std::option::Option<std::vec::Vec<ConnectRoutingProfileQueueConfigsEl>>,
    dynamic: ConnectRoutingProfileDynamic,
}

struct ConnectRoutingProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectRoutingProfileData>,
}

#[derive(Clone)]
pub struct ConnectRoutingProfile(std::rc::Rc<ConnectRoutingProfile_>);

impl ConnectRoutingProfile {
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

    #[doc = "Set the field `media_concurrencies`.\n"]
    pub fn set_media_concurrencies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectRoutingProfileMediaConcurrenciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_concurrencies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_concurrencies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `queue_configs`.\n"]
    pub fn set_queue_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectRoutingProfileQueueConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue_configs = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_configs_associated` after provisioning.\n"]
    pub fn queue_configs_associated(&self) -> terrars::SetRef<ConnectRoutingProfileQueueConfigsAssociatedElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queue_configs_associated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectRoutingProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectRoutingProfile { }

impl terrars::ToListMappable for ConnectRoutingProfile {
    type O = terrars::ListRef<ConnectRoutingProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectRoutingProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_routing_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectRoutingProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub default_outbound_queue_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectRoutingProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectRoutingProfile {
        let out = ConnectRoutingProfile(std::rc::Rc::new(ConnectRoutingProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectRoutingProfileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_outbound_queue_id: self.default_outbound_queue_id,
                description: self.description,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                media_concurrencies: core::default::Default::default(),
                queue_configs: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectRoutingProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectRoutingProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectRoutingProfileRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_configs_associated` after provisioning.\n"]
    pub fn queue_configs_associated(&self) -> terrars::SetRef<ConnectRoutingProfileQueueConfigsAssociatedElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.queue_configs_associated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectRoutingProfileQueueConfigsAssociatedEl {
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

impl ConnectRoutingProfileQueueConfigsAssociatedEl {
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

impl terrars::ToListMappable for ConnectRoutingProfileQueueConfigsAssociatedEl {
    type O = terrars::BlockAssignable<ConnectRoutingProfileQueueConfigsAssociatedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectRoutingProfileQueueConfigsAssociatedEl {}

impl BuildConnectRoutingProfileQueueConfigsAssociatedEl {
    pub fn build(self) -> ConnectRoutingProfileQueueConfigsAssociatedEl {
        ConnectRoutingProfileQueueConfigsAssociatedEl {
            channel: core::default::Default::default(),
            delay: core::default::Default::default(),
            priority: core::default::Default::default(),
            queue_arn: core::default::Default::default(),
            queue_id: core::default::Default::default(),
            queue_name: core::default::Default::default(),
        }
    }
}

pub struct ConnectRoutingProfileQueueConfigsAssociatedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectRoutingProfileQueueConfigsAssociatedElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectRoutingProfileQueueConfigsAssociatedElRef {
        ConnectRoutingProfileQueueConfigsAssociatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectRoutingProfileQueueConfigsAssociatedElRef {
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

#[derive(Serialize)]
pub struct ConnectRoutingProfileMediaConcurrenciesEl {
    channel: terrars::PrimField<std::string::String>,
    concurrency: terrars::PrimField<f64>,
}

impl ConnectRoutingProfileMediaConcurrenciesEl { }

impl terrars::ToListMappable for ConnectRoutingProfileMediaConcurrenciesEl {
    type O = terrars::BlockAssignable<ConnectRoutingProfileMediaConcurrenciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectRoutingProfileMediaConcurrenciesEl {
    #[doc = ""]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub concurrency: terrars::PrimField<f64>,
}

impl BuildConnectRoutingProfileMediaConcurrenciesEl {
    pub fn build(self) -> ConnectRoutingProfileMediaConcurrenciesEl {
        ConnectRoutingProfileMediaConcurrenciesEl {
            channel: self.channel,
            concurrency: self.concurrency,
        }
    }
}

pub struct ConnectRoutingProfileMediaConcurrenciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectRoutingProfileMediaConcurrenciesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectRoutingProfileMediaConcurrenciesElRef {
        ConnectRoutingProfileMediaConcurrenciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectRoutingProfileMediaConcurrenciesElRef {
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
}

#[derive(Serialize)]
pub struct ConnectRoutingProfileQueueConfigsEl {
    channel: terrars::PrimField<std::string::String>,
    delay: terrars::PrimField<f64>,
    priority: terrars::PrimField<f64>,
    queue_id: terrars::PrimField<std::string::String>,
}

impl ConnectRoutingProfileQueueConfigsEl { }

impl terrars::ToListMappable for ConnectRoutingProfileQueueConfigsEl {
    type O = terrars::BlockAssignable<ConnectRoutingProfileQueueConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectRoutingProfileQueueConfigsEl {
    #[doc = ""]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub delay: terrars::PrimField<f64>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub queue_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectRoutingProfileQueueConfigsEl {
    pub fn build(self) -> ConnectRoutingProfileQueueConfigsEl {
        ConnectRoutingProfileQueueConfigsEl {
            channel: self.channel,
            delay: self.delay,
            priority: self.priority,
            queue_id: self.queue_id,
        }
    }
}

pub struct ConnectRoutingProfileQueueConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectRoutingProfileQueueConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectRoutingProfileQueueConfigsElRef {
        ConnectRoutingProfileQueueConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectRoutingProfileQueueConfigsElRef {
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

#[derive(Serialize, Default)]
struct ConnectRoutingProfileDynamic {
    media_concurrencies: std::option::Option<terrars::DynamicBlock<ConnectRoutingProfileMediaConcurrenciesEl>>,
    queue_configs: std::option::Option<terrars::DynamicBlock<ConnectRoutingProfileQueueConfigsEl>>,
}
