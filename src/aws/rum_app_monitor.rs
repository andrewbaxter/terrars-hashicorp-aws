use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RumAppMonitorData {
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
    cw_log_enabled: std::option::Option<terrars::PrimField<bool>>,
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_monitor_configuration: std::option::Option<std::vec::Vec<RumAppMonitorAppMonitorConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_events: std::option::Option<std::vec::Vec<RumAppMonitorCustomEventsEl>>,
    dynamic: RumAppMonitorDynamic,
}

struct RumAppMonitor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RumAppMonitorData>,
}

#[derive(Clone)]
pub struct RumAppMonitor(std::rc::Rc<RumAppMonitor_>);

impl RumAppMonitor {
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

    #[doc = "Set the field `cw_log_enabled`.\n"]
    pub fn set_cw_log_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cw_log_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `app_monitor_configuration`.\n"]
    pub fn set_app_monitor_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RumAppMonitorAppMonitorConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_monitor_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_monitor_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_events`.\n"]
    pub fn set_custom_events(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RumAppMonitorCustomEventsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_events = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_events = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_monitor_id` after provisioning.\n"]
    pub fn app_monitor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_monitor_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_enabled` after provisioning.\n"]
    pub fn cw_log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cw_log_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_group` after provisioning.\n"]
    pub fn cw_log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cw_log_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_monitor_configuration` after provisioning.\n"]
    pub fn app_monitor_configuration(&self) -> terrars::ListRef<RumAppMonitorAppMonitorConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_monitor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_events` after provisioning.\n"]
    pub fn custom_events(&self) -> terrars::ListRef<RumAppMonitorCustomEventsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_events", self.extract_ref()))
    }
}

impl terrars::Referable for RumAppMonitor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RumAppMonitor { }

impl terrars::ToListMappable for RumAppMonitor {
    type O = terrars::ListRef<RumAppMonitorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RumAppMonitor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_rum_app_monitor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRumAppMonitor {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRumAppMonitor {
    pub fn build(self, stack: &mut terrars::Stack) -> RumAppMonitor {
        let out = RumAppMonitor(std::rc::Rc::new(RumAppMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RumAppMonitorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cw_log_enabled: core::default::Default::default(),
                domain: self.domain,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                app_monitor_configuration: core::default::Default::default(),
                custom_events: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RumAppMonitorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RumAppMonitorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RumAppMonitorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_monitor_id` after provisioning.\n"]
    pub fn app_monitor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_monitor_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_enabled` after provisioning.\n"]
    pub fn cw_log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cw_log_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cw_log_group` after provisioning.\n"]
    pub fn cw_log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cw_log_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_monitor_configuration` after provisioning.\n"]
    pub fn app_monitor_configuration(&self) -> terrars::ListRef<RumAppMonitorAppMonitorConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_monitor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_events` after provisioning.\n"]
    pub fn custom_events(&self) -> terrars::ListRef<RumAppMonitorCustomEventsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_events", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RumAppMonitorAppMonitorConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_cookies: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_xray: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_pages: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    favorite_pages: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_pages: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_sample_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telemetries: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl RumAppMonitorAppMonitorConfigurationEl {
    #[doc = "Set the field `allow_cookies`.\n"]
    pub fn set_allow_cookies(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_xray`.\n"]
    pub fn set_enable_xray(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_xray = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_pages`.\n"]
    pub fn set_excluded_pages(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_pages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `favorite_pages`.\n"]
    pub fn set_favorite_pages(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.favorite_pages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_role_arn`.\n"]
    pub fn set_guest_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guest_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `included_pages`.\n"]
    pub fn set_included_pages(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_pages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_sample_rate`.\n"]
    pub fn set_session_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_sample_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `telemetries`.\n"]
    pub fn set_telemetries(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.telemetries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RumAppMonitorAppMonitorConfigurationEl {
    type O = terrars::BlockAssignable<RumAppMonitorAppMonitorConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRumAppMonitorAppMonitorConfigurationEl {}

impl BuildRumAppMonitorAppMonitorConfigurationEl {
    pub fn build(self) -> RumAppMonitorAppMonitorConfigurationEl {
        RumAppMonitorAppMonitorConfigurationEl {
            allow_cookies: core::default::Default::default(),
            enable_xray: core::default::Default::default(),
            excluded_pages: core::default::Default::default(),
            favorite_pages: core::default::Default::default(),
            guest_role_arn: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            included_pages: core::default::Default::default(),
            session_sample_rate: core::default::Default::default(),
            telemetries: core::default::Default::default(),
        }
    }
}

pub struct RumAppMonitorAppMonitorConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RumAppMonitorAppMonitorConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RumAppMonitorAppMonitorConfigurationElRef {
        RumAppMonitorAppMonitorConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RumAppMonitorAppMonitorConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_cookies` after provisioning.\n"]
    pub fn allow_cookies(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_xray` after provisioning.\n"]
    pub fn enable_xray(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_xray", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_pages` after provisioning.\n"]
    pub fn excluded_pages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `favorite_pages` after provisioning.\n"]
    pub fn favorite_pages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.favorite_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `guest_role_arn` after provisioning.\n"]
    pub fn guest_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `included_pages` after provisioning.\n"]
    pub fn included_pages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.included_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `session_sample_rate` after provisioning.\n"]
    pub fn session_sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_sample_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `telemetries` after provisioning.\n"]
    pub fn telemetries(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.telemetries", self.base))
    }
}

#[derive(Serialize)]
pub struct RumAppMonitorCustomEventsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RumAppMonitorCustomEventsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RumAppMonitorCustomEventsEl {
    type O = terrars::BlockAssignable<RumAppMonitorCustomEventsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRumAppMonitorCustomEventsEl {}

impl BuildRumAppMonitorCustomEventsEl {
    pub fn build(self) -> RumAppMonitorCustomEventsEl {
        RumAppMonitorCustomEventsEl { status: core::default::Default::default() }
    }
}

pub struct RumAppMonitorCustomEventsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RumAppMonitorCustomEventsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RumAppMonitorCustomEventsElRef {
        RumAppMonitorCustomEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RumAppMonitorCustomEventsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct RumAppMonitorDynamic {
    app_monitor_configuration: std::option::Option<terrars::DynamicBlock<RumAppMonitorAppMonitorConfigurationEl>>,
    custom_events: std::option::Option<terrars::DynamicBlock<RumAppMonitorCustomEventsEl>>,
}
