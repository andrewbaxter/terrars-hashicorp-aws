use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2ConfigurationSetData {
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
    configuration_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_options: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetDeliveryOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reputation_options: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetReputationOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_options: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetSendingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppression_options: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracking_options: std::option::Option<std::vec::Vec<Sesv2ConfigurationSetTrackingOptionsEl>>,
    dynamic: Sesv2ConfigurationSetDynamic,
}

struct Sesv2ConfigurationSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Sesv2ConfigurationSetData>,
}

#[derive(Clone)]
pub struct Sesv2ConfigurationSet(std::rc::Rc<Sesv2ConfigurationSet_>);

impl Sesv2ConfigurationSet {
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

    #[doc = "Set the field `delivery_options`.\n"]
    pub fn set_delivery_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2ConfigurationSetDeliveryOptionsEl>>,
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

    #[doc = "Set the field `reputation_options`.\n"]
    pub fn set_reputation_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2ConfigurationSetReputationOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reputation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reputation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sending_options`.\n"]
    pub fn set_sending_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2ConfigurationSetSendingOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sending_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sending_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `suppression_options`.\n"]
    pub fn set_suppression_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().suppression_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.suppression_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tracking_options`.\n"]
    pub fn set_tracking_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2ConfigurationSetTrackingOptionsEl>>,
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetDeliveryOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetReputationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reputation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetSendingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sending_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetSuppressionOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppression_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetTrackingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }
}

impl terrars::Referable for Sesv2ConfigurationSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Sesv2ConfigurationSet { }

impl terrars::ToListMappable for Sesv2ConfigurationSet {
    type O = terrars::ListRef<Sesv2ConfigurationSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Sesv2ConfigurationSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sesv2_configuration_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2ConfigurationSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub configuration_set_name: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSet {
    pub fn build(self, stack: &mut terrars::Stack) -> Sesv2ConfigurationSet {
        let out = Sesv2ConfigurationSet(std::rc::Rc::new(Sesv2ConfigurationSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Sesv2ConfigurationSetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                configuration_set_name: self.configuration_set_name,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                delivery_options: core::default::Default::default(),
                reputation_options: core::default::Default::default(),
                sending_options: core::default::Default::default(),
                suppression_options: core::default::Default::default(),
                tracking_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2ConfigurationSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Sesv2ConfigurationSetRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_options` after provisioning.\n"]
    pub fn delivery_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetDeliveryOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reputation_options` after provisioning.\n"]
    pub fn reputation_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetReputationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reputation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sending_options` after provisioning.\n"]
    pub fn sending_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetSendingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sending_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suppression_options` after provisioning.\n"]
    pub fn suppression_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetSuppressionOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppression_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_options` after provisioning.\n"]
    pub fn tracking_options(&self) -> terrars::ListRef<Sesv2ConfigurationSetTrackingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetDeliveryOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_pool_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2ConfigurationSetDeliveryOptionsEl {
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

impl terrars::ToListMappable for Sesv2ConfigurationSetDeliveryOptionsEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetDeliveryOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetDeliveryOptionsEl {}

impl BuildSesv2ConfigurationSetDeliveryOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetDeliveryOptionsEl {
        Sesv2ConfigurationSetDeliveryOptionsEl {
            sending_pool_name: core::default::Default::default(),
            tls_policy: core::default::Default::default(),
        }
    }
}

pub struct Sesv2ConfigurationSetDeliveryOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetDeliveryOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2ConfigurationSetDeliveryOptionsElRef {
        Sesv2ConfigurationSetDeliveryOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetDeliveryOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct Sesv2ConfigurationSetReputationOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reputation_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Sesv2ConfigurationSetReputationOptionsEl {
    #[doc = "Set the field `reputation_metrics_enabled`.\n"]
    pub fn set_reputation_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reputation_metrics_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2ConfigurationSetReputationOptionsEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetReputationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetReputationOptionsEl {}

impl BuildSesv2ConfigurationSetReputationOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetReputationOptionsEl {
        Sesv2ConfigurationSetReputationOptionsEl { reputation_metrics_enabled: core::default::Default::default() }
    }
}

pub struct Sesv2ConfigurationSetReputationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetReputationOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2ConfigurationSetReputationOptionsElRef {
        Sesv2ConfigurationSetReputationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetReputationOptionsElRef {
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
pub struct Sesv2ConfigurationSetSendingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sending_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Sesv2ConfigurationSetSendingOptionsEl {
    #[doc = "Set the field `sending_enabled`.\n"]
    pub fn set_sending_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sending_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2ConfigurationSetSendingOptionsEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetSendingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetSendingOptionsEl {}

impl BuildSesv2ConfigurationSetSendingOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetSendingOptionsEl {
        Sesv2ConfigurationSetSendingOptionsEl { sending_enabled: core::default::Default::default() }
    }
}

pub struct Sesv2ConfigurationSetSendingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetSendingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2ConfigurationSetSendingOptionsElRef {
        Sesv2ConfigurationSetSendingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetSendingOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sending_enabled` after provisioning.\n"]
    pub fn sending_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sending_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetSuppressionOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppressed_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Sesv2ConfigurationSetSuppressionOptionsEl {
    #[doc = "Set the field `suppressed_reasons`.\n"]
    pub fn set_suppressed_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.suppressed_reasons = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2ConfigurationSetSuppressionOptionsEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetSuppressionOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetSuppressionOptionsEl {}

impl BuildSesv2ConfigurationSetSuppressionOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetSuppressionOptionsEl {
        Sesv2ConfigurationSetSuppressionOptionsEl { suppressed_reasons: core::default::Default::default() }
    }
}

pub struct Sesv2ConfigurationSetSuppressionOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetSuppressionOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2ConfigurationSetSuppressionOptionsElRef {
        Sesv2ConfigurationSetSuppressionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetSuppressionOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `suppressed_reasons` after provisioning.\n"]
    pub fn suppressed_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suppressed_reasons", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2ConfigurationSetTrackingOptionsEl {
    custom_redirect_domain: terrars::PrimField<std::string::String>,
}

impl Sesv2ConfigurationSetTrackingOptionsEl { }

impl terrars::ToListMappable for Sesv2ConfigurationSetTrackingOptionsEl {
    type O = terrars::BlockAssignable<Sesv2ConfigurationSetTrackingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2ConfigurationSetTrackingOptionsEl {
    #[doc = ""]
    pub custom_redirect_domain: terrars::PrimField<std::string::String>,
}

impl BuildSesv2ConfigurationSetTrackingOptionsEl {
    pub fn build(self) -> Sesv2ConfigurationSetTrackingOptionsEl {
        Sesv2ConfigurationSetTrackingOptionsEl { custom_redirect_domain: self.custom_redirect_domain }
    }
}

pub struct Sesv2ConfigurationSetTrackingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2ConfigurationSetTrackingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2ConfigurationSetTrackingOptionsElRef {
        Sesv2ConfigurationSetTrackingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2ConfigurationSetTrackingOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_redirect_domain` after provisioning.\n"]
    pub fn custom_redirect_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_redirect_domain", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2ConfigurationSetDynamic {
    delivery_options: std::option::Option<terrars::DynamicBlock<Sesv2ConfigurationSetDeliveryOptionsEl>>,
    reputation_options: std::option::Option<terrars::DynamicBlock<Sesv2ConfigurationSetReputationOptionsEl>>,
    sending_options: std::option::Option<terrars::DynamicBlock<Sesv2ConfigurationSetSendingOptionsEl>>,
    suppression_options: std::option::Option<terrars::DynamicBlock<Sesv2ConfigurationSetSuppressionOptionsEl>>,
    tracking_options: std::option::Option<terrars::DynamicBlock<Sesv2ConfigurationSetTrackingOptionsEl>>,
}
