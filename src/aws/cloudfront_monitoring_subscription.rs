use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontMonitoringSubscriptionData {
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
    distribution_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_subscription: std::option::Option<std::vec::Vec<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
    dynamic: CloudfrontMonitoringSubscriptionDynamic,
}

struct CloudfrontMonitoringSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontMonitoringSubscriptionData>,
}

#[derive(Clone)]
pub struct CloudfrontMonitoringSubscription(std::rc::Rc<CloudfrontMonitoringSubscription_>);

impl CloudfrontMonitoringSubscription {
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

    #[doc = "Set the field `monitoring_subscription`.\n"]
    pub fn set_monitoring_subscription(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_subscription = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_subscription = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_subscription` after provisioning.\n"]
    pub fn monitoring_subscription(
        &self,
    ) -> terrars::ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_subscription", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontMonitoringSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontMonitoringSubscription { }

impl terrars::ToListMappable for CloudfrontMonitoringSubscription {
    type O = terrars::ListRef<CloudfrontMonitoringSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontMonitoringSubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_monitoring_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontMonitoringSubscription {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub distribution_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMonitoringSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontMonitoringSubscription {
        let out = CloudfrontMonitoringSubscription(std::rc::Rc::new(CloudfrontMonitoringSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontMonitoringSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                distribution_id: self.distribution_id,
                id: core::default::Default::default(),
                monitoring_subscription: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontMonitoringSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMonitoringSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontMonitoringSubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `distribution_id` after provisioning.\n"]
    pub fn distribution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_subscription` after provisioning.\n"]
    pub fn monitoring_subscription(
        &self,
    ) -> terrars::ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_subscription", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
    realtime_metrics_subscription_status: terrars::PrimField<std::string::String>,
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl { }

impl terrars
::ToListMappable for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
    #[doc = ""]
    pub realtime_metrics_subscription_status: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
    pub fn build(
        self,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl {
            realtime_metrics_subscription_status: self.realtime_metrics_subscription_status,
        }
    }
}

pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `realtime_metrics_subscription_status` after provisioning.\n"]
    pub fn realtime_metrics_subscription_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realtime_metrics_subscription_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElDynamic {
    realtime_metrics_subscription_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realtime_metrics_subscription_config: std
    ::option
    ::Option<
        std::vec::Vec<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl>,
    >,
    dynamic: CloudfrontMonitoringSubscriptionMonitoringSubscriptionElDynamic,
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    #[doc = "Set the field `realtime_metrics_subscription_config`.\n"]
    pub fn set_realtime_metrics_subscription_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.realtime_metrics_subscription_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.realtime_metrics_subscription_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    type O = terrars::BlockAssignable<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {}

impl BuildCloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
    pub fn build(self) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl {
            realtime_metrics_subscription_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
        CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `realtime_metrics_subscription_config` after provisioning.\n"]
    pub fn realtime_metrics_subscription_config(
        &self,
    ) -> terrars
    ::ListRef<CloudfrontMonitoringSubscriptionMonitoringSubscriptionElRealtimeMetricsSubscriptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.realtime_metrics_subscription_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMonitoringSubscriptionDynamic {
    monitoring_subscription: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMonitoringSubscriptionMonitoringSubscriptionEl>>,
}
