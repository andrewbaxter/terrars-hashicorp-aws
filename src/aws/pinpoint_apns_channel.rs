use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PinpointApnsChannelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_authentication_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_key_id: Option<PrimField<String>>,
}

struct PinpointApnsChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PinpointApnsChannelData>,
}

#[derive(Clone)]
pub struct PinpointApnsChannel(Rc<PinpointApnsChannel_>);

impl PinpointApnsChannel {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `bundle_id`.\n"]
    pub fn set_bundle_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bundle_id = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `default_authentication_method`.\n"]
    pub fn set_default_authentication_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_authentication_method = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `private_key`.\n"]
    pub fn set_private_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_key = Some(v.into());
        self
    }

    #[doc= "Set the field `team_id`.\n"]
    pub fn set_team_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().team_id = Some(v.into());
        self
    }

    #[doc= "Set the field `token_key`.\n"]
    pub fn set_token_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token_key = Some(v.into());
        self
    }

    #[doc= "Set the field `token_key_id`.\n"]
    pub fn set_token_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token_key_id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_authentication_method` after provisioning.\n"]
    pub fn default_authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_authentication_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\n"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token_key` after provisioning.\n"]
    pub fn token_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token_key_id` after provisioning.\n"]
    pub fn token_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_key_id", self.extract_ref()))
    }
}

impl Resource for PinpointApnsChannel {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for PinpointApnsChannel {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for PinpointApnsChannel {
    type O = ListRef<PinpointApnsChannelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for PinpointApnsChannel_ {
    fn extract_resource_type(&self) -> String {
        "aws_pinpoint_apns_channel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointApnsChannel {
    pub tf_id: String,
    #[doc= ""]
    pub application_id: PrimField<String>,
}

impl BuildPinpointApnsChannel {
    pub fn build(self, stack: &mut Stack) -> PinpointApnsChannel {
        let out = PinpointApnsChannel(Rc::new(PinpointApnsChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PinpointApnsChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_id: self.application_id,
                bundle_id: core::default::Default::default(),
                certificate: core::default::Default::default(),
                default_authentication_method: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                private_key: core::default::Default::default(),
                team_id: core::default::Default::default(),
                token_key: core::default::Default::default(),
                token_key_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PinpointApnsChannelRef {
    shared: StackShared,
    base: String,
}

impl Ref for PinpointApnsChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PinpointApnsChannelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_authentication_method` after provisioning.\n"]
    pub fn default_authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_authentication_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\n"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token_key` after provisioning.\n"]
    pub fn token_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token_key_id` after provisioning.\n"]
    pub fn token_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_key_id", self.extract_ref()))
    }
}
