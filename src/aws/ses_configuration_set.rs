use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SesConfigurationSetData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reputation_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_options: std::option::Option<std::vec::Vec<SesConfigurationSetDeliveryOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracking_options: std::option::Option<std::vec::Vec<SesConfigurationSetTrackingOptionsEl>>,
    dynamic: SesConfigurationSetDynamic,
}

struct SesConfigurationSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SesConfigurationSetData>,
}

#[derive(Clone)]
pub struct SesConfigurationSet(std::rc::Rc<SesConfigurationSet_>);

impl SesConfigurationSet {
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

    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reputation_metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sending_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_options`.\n"]
    pub fn set_delivery_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesConfigurationSetDeliveryOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delivery_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delivery_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tracking_options`.\n"]
    pub fn set_tracking_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesConfigurationSetTrackingOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tracking_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tracking_options = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_fresh_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reputation_metrics_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sending_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<SesConfigurationSetDeliveryOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<SesConfigurationSetTrackingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }
}

impl terrars::Referable for SesConfigurationSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SesConfigurationSet { }

impl terrars::ToListMappable for SesConfigurationSet {
    type O = terrars::ListRef<SesConfigurationSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SesConfigurationSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ses_configuration_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesConfigurationSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSesConfigurationSet {
    pub fn build(self, stack: &mut terrars::Stack) -> SesConfigurationSet {
        let out = SesConfigurationSet(std::rc::Rc::new(SesConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SesConfigurationSetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                reputation_metrics_enabled: core::default::Default::default(),
                sending_enabled: core::default::Default::default(),
                delivery_options: core::default::Default::default(),
                tracking_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SesConfigurationSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesConfigurationSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SesConfigurationSetRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_fresh_start` after provisioning.\n"]
    pub fn last_fresh_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_fresh_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reputation_metrics_enabled` after provisioning.\n"]
    pub fn reputation_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reputation_metrics_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sending_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<SesConfigurationSetDeliveryOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<SesConfigurationSetTrackingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SesConfigurationSetDeliveryOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesConfigurationSetDeliveryOptionsEl {
    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesConfigurationSetDeliveryOptionsEl {
    type O = terrars::BlockAssignable<SesConfigurationSetDeliveryOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesConfigurationSetDeliveryOptionsEl {}

impl BuildSesConfigurationSetDeliveryOptionsEl {
    pub fn build(self) -> SesConfigurationSetDeliveryOptionsEl {
        SesConfigurationSetDeliveryOptionsEl { tls_policy: core::default::Default::default() }
    }
}

pub struct SesConfigurationSetDeliveryOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesConfigurationSetDeliveryOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesConfigurationSetDeliveryOptionsElRef {
        SesConfigurationSetDeliveryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesConfigurationSetDeliveryOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct SesConfigurationSetTrackingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_redirect_domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesConfigurationSetTrackingOptionsEl {
    #[doc = "Set the field `custom_redirect_domain`.\n"]
    pub fn set_custom_redirect_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_redirect_domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesConfigurationSetTrackingOptionsEl {
    type O = terrars::BlockAssignable<SesConfigurationSetTrackingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesConfigurationSetTrackingOptionsEl {}

impl BuildSesConfigurationSetTrackingOptionsEl {
    pub fn build(self) -> SesConfigurationSetTrackingOptionsEl {
        SesConfigurationSetTrackingOptionsEl { custom_redirect_domain: core::default::Default::default() }
    }
}

pub struct SesConfigurationSetTrackingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesConfigurationSetTrackingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesConfigurationSetTrackingOptionsElRef {
        SesConfigurationSetTrackingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesConfigurationSetTrackingOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_redirect_domain` after provisioning.\n"]
    pub fn custom_redirect_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_redirect_domain", self.base))
    }
}

#[derive(Serialize, Default)]
struct SesConfigurationSetDynamic {
    delivery_options: std::option::Option<terrars::DynamicBlock<SesConfigurationSetDeliveryOptionsEl>>,
    tracking_options: std::option::Option<terrars::DynamicBlock<SesConfigurationSetTrackingOptionsEl>>,
}
