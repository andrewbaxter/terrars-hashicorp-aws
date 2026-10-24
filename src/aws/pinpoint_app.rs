use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PinpointAppData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    campaign_hook: std::option::Option<std::vec::Vec<PinpointAppCampaignHookEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<PinpointAppLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quiet_time: std::option::Option<std::vec::Vec<PinpointAppQuietTimeEl>>,
    dynamic: PinpointAppDynamic,
}

struct PinpointApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PinpointAppData>,
}

#[derive(Clone)]
pub struct PinpointApp(std::rc::Rc<PinpointApp_>);

impl PinpointApp {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `campaign_hook`.\n"]
    pub fn set_campaign_hook(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PinpointAppCampaignHookEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().campaign_hook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.campaign_hook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(self, v: impl std::convert::Into<terrars::BlockAssignable<PinpointAppLimitsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quiet_time`.\n"]
    pub fn set_quiet_time(self, v: impl std::convert::Into<terrars::BlockAssignable<PinpointAppQuietTimeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quiet_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quiet_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `campaign_hook` after provisioning.\n"]
    pub fn campaign_hook(&self) -> terrars::ListRef<PinpointAppCampaignHookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.campaign_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<PinpointAppLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quiet_time` after provisioning.\n"]
    pub fn quiet_time(&self) -> terrars::ListRef<PinpointAppQuietTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quiet_time", self.extract_ref()))
    }
}

impl terrars::Referable for PinpointApp {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PinpointApp { }

impl terrars::ToListMappable for PinpointApp {
    type O = terrars::ListRef<PinpointAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PinpointApp_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_pinpoint_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointApp {
    pub tf_id: std::string::String,
}

impl BuildPinpointApp {
    pub fn build(self, stack: &mut terrars::Stack) -> PinpointApp {
        let out = PinpointApp(std::rc::Rc::new(PinpointApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PinpointAppData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                campaign_hook: core::default::Default::default(),
                limits: core::default::Default::default(),
                quiet_time: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PinpointAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PinpointAppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `campaign_hook` after provisioning.\n"]
    pub fn campaign_hook(&self) -> terrars::ListRef<PinpointAppCampaignHookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.campaign_hook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<PinpointAppLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quiet_time` after provisioning.\n"]
    pub fn quiet_time(&self) -> terrars::ListRef<PinpointAppQuietTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quiet_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PinpointAppCampaignHookEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PinpointAppCampaignHookEl {
    #[doc = "Set the field `lambda_function_name`.\n"]
    pub fn set_lambda_function_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lambda_function_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PinpointAppCampaignHookEl {
    type O = terrars::BlockAssignable<PinpointAppCampaignHookEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppCampaignHookEl {}

impl BuildPinpointAppCampaignHookEl {
    pub fn build(self) -> PinpointAppCampaignHookEl {
        PinpointAppCampaignHookEl {
            lambda_function_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppCampaignHookElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointAppCampaignHookElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PinpointAppCampaignHookElRef {
        PinpointAppCampaignHookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppCampaignHookElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_function_name` after provisioning.\n"]
    pub fn lambda_function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_function_name", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}

#[derive(Serialize)]
pub struct PinpointAppLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total: std::option::Option<terrars::PrimField<f64>>,
}

impl PinpointAppLimitsEl {
    #[doc = "Set the field `daily`.\n"]
    pub fn set_daily(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.daily = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_duration`.\n"]
    pub fn set_maximum_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `messages_per_second`.\n"]
    pub fn set_messages_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.messages_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total`.\n"]
    pub fn set_total(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PinpointAppLimitsEl {
    type O = terrars::BlockAssignable<PinpointAppLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppLimitsEl {}

impl BuildPinpointAppLimitsEl {
    pub fn build(self) -> PinpointAppLimitsEl {
        PinpointAppLimitsEl {
            daily: core::default::Default::default(),
            maximum_duration: core::default::Default::default(),
            messages_per_second: core::default::Default::default(),
            total: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointAppLimitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PinpointAppLimitsElRef {
        PinpointAppLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily` after provisioning.\n"]
    pub fn daily(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daily", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_duration` after provisioning.\n"]
    pub fn maximum_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `messages_per_second` after provisioning.\n"]
    pub fn messages_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.messages_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `total` after provisioning.\n"]
    pub fn total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total", self.base))
    }
}

#[derive(Serialize)]
pub struct PinpointAppQuietTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PinpointAppQuietTimeEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PinpointAppQuietTimeEl {
    type O = terrars::BlockAssignable<PinpointAppQuietTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointAppQuietTimeEl {}

impl BuildPinpointAppQuietTimeEl {
    pub fn build(self) -> PinpointAppQuietTimeEl {
        PinpointAppQuietTimeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct PinpointAppQuietTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PinpointAppQuietTimeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PinpointAppQuietTimeElRef {
        PinpointAppQuietTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PinpointAppQuietTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct PinpointAppDynamic {
    campaign_hook: std::option::Option<terrars::DynamicBlock<PinpointAppCampaignHookEl>>,
    limits: std::option::Option<terrars::DynamicBlock<PinpointAppLimitsEl>>,
    quiet_time: std::option::Option<terrars::DynamicBlock<PinpointAppQuietTimeEl>>,
}
