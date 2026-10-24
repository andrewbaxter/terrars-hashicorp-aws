use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbTargetGroupData {
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
    connection_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deregistration_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_multi_value_headers_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_algorithm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_client_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_protocol_v2: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slow_start: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<LbTargetGroupHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<LbTargetGroupStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_failover: std::option::Option<std::vec::Vec<LbTargetGroupTargetFailoverEl>>,
    dynamic: LbTargetGroupDynamic,
}

struct LbTargetGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LbTargetGroupData>,
}

#[derive(Clone)]
pub struct LbTargetGroup(std::rc::Rc<LbTargetGroup_>);

impl LbTargetGroup {
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

    #[doc = "Set the field `connection_termination`.\n"]
    pub fn set_connection_termination(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connection_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deregistration_delay`.\n"]
    pub fn set_deregistration_delay(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deregistration_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_multi_value_headers_enabled`.\n"]
    pub fn set_lambda_multi_value_headers_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lambda_multi_value_headers_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancing_algorithm_type`.\n"]
    pub fn set_load_balancing_algorithm_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().load_balancing_algorithm_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_client_ip`.\n"]
    pub fn set_preserve_client_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preserve_client_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol_version`.\n"]
    pub fn set_protocol_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_protocol_v2`.\n"]
    pub fn set_proxy_protocol_v2(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxy_protocol_v2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slow_start`.\n"]
    pub fn set_slow_start(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().slow_start = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbTargetGroupHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbTargetGroupStickinessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stickiness = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stickiness = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_failover`.\n"]
    pub fn set_target_failover(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbTargetGroupTargetFailoverEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_failover = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_failover = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_termination` after provisioning.\n"]
    pub fn connection_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregistration_delay` after provisioning.\n"]
    pub fn deregistration_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deregistration_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_multi_value_headers_enabled` after provisioning.\n"]
    pub fn lambda_multi_value_headers_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_multi_value_headers_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancing_algorithm_type` after provisioning.\n"]
    pub fn load_balancing_algorithm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_algorithm_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_client_ip` after provisioning.\n"]
    pub fn preserve_client_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_client_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_v2` after provisioning.\n"]
    pub fn proxy_protocol_v2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slow_start` after provisioning.\n"]
    pub fn slow_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slow_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<LbTargetGroupHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbTargetGroupStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_failover` after provisioning.\n"]
    pub fn target_failover(&self) -> terrars::ListRef<LbTargetGroupTargetFailoverElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_failover", self.extract_ref()))
    }
}

impl terrars::Referable for LbTargetGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LbTargetGroup { }

impl terrars::ToListMappable for LbTargetGroup {
    type O = terrars::ListRef<LbTargetGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LbTargetGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lb_target_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbTargetGroup {
    pub tf_id: std::string::String,
}

impl BuildLbTargetGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> LbTargetGroup {
        let out = LbTargetGroup(std::rc::Rc::new(LbTargetGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LbTargetGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_termination: core::default::Default::default(),
                deregistration_delay: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                lambda_multi_value_headers_enabled: core::default::Default::default(),
                load_balancing_algorithm_type: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                port: core::default::Default::default(),
                preserve_client_ip: core::default::Default::default(),
                protocol: core::default::Default::default(),
                protocol_version: core::default::Default::default(),
                proxy_protocol_v2: core::default::Default::default(),
                slow_start: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_type: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                health_check: core::default::Default::default(),
                stickiness: core::default::Default::default(),
                target_failover: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbTargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbTargetGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LbTargetGroupRef {
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

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_termination` after provisioning.\n"]
    pub fn connection_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregistration_delay` after provisioning.\n"]
    pub fn deregistration_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deregistration_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_multi_value_headers_enabled` after provisioning.\n"]
    pub fn lambda_multi_value_headers_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_multi_value_headers_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancing_algorithm_type` after provisioning.\n"]
    pub fn load_balancing_algorithm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_algorithm_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_client_ip` after provisioning.\n"]
    pub fn preserve_client_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_client_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\n"]
    pub fn protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_v2` after provisioning.\n"]
    pub fn proxy_protocol_v2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_protocol_v2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `slow_start` after provisioning.\n"]
    pub fn slow_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slow_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<LbTargetGroupHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbTargetGroupStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_failover` after provisioning.\n"]
    pub fn target_failover(&self) -> terrars::ListRef<LbTargetGroupTargetFailoverElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_failover", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbTargetGroupHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matcher: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl LbTargetGroupHealthCheckEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matcher`.\n"]
    pub fn set_matcher(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.matcher = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbTargetGroupHealthCheckEl {
    type O = terrars::BlockAssignable<LbTargetGroupHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbTargetGroupHealthCheckEl {}

impl BuildLbTargetGroupHealthCheckEl {
    pub fn build(self) -> LbTargetGroupHealthCheckEl {
        LbTargetGroupHealthCheckEl {
            enabled: core::default::Default::default(),
            healthy_threshold: core::default::Default::default(),
            interval: core::default::Default::default(),
            matcher: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

pub struct LbTargetGroupHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbTargetGroupHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbTargetGroupHealthCheckElRef {
        LbTargetGroupHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbTargetGroupHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `matcher` after provisioning.\n"]
    pub fn matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matcher", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct LbTargetGroupStickinessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl LbTargetGroupStickinessEl {
    #[doc = "Set the field `cookie_duration`.\n"]
    pub fn set_cookie_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cookie_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_name`.\n"]
    pub fn set_cookie_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbTargetGroupStickinessEl {
    type O = terrars::BlockAssignable<LbTargetGroupStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbTargetGroupStickinessEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbTargetGroupStickinessEl {
    pub fn build(self) -> LbTargetGroupStickinessEl {
        LbTargetGroupStickinessEl {
            cookie_duration: core::default::Default::default(),
            cookie_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct LbTargetGroupStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbTargetGroupStickinessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbTargetGroupStickinessElRef {
        LbTargetGroupStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbTargetGroupStickinessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_duration` after provisioning.\n"]
    pub fn cookie_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_name` after provisioning.\n"]
    pub fn cookie_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct LbTargetGroupTargetFailoverEl {
    on_deregistration: terrars::PrimField<std::string::String>,
    on_unhealthy: terrars::PrimField<std::string::String>,
}

impl LbTargetGroupTargetFailoverEl { }

impl terrars::ToListMappable for LbTargetGroupTargetFailoverEl {
    type O = terrars::BlockAssignable<LbTargetGroupTargetFailoverEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbTargetGroupTargetFailoverEl {
    #[doc = ""]
    pub on_deregistration: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub on_unhealthy: terrars::PrimField<std::string::String>,
}

impl BuildLbTargetGroupTargetFailoverEl {
    pub fn build(self) -> LbTargetGroupTargetFailoverEl {
        LbTargetGroupTargetFailoverEl {
            on_deregistration: self.on_deregistration,
            on_unhealthy: self.on_unhealthy,
        }
    }
}

pub struct LbTargetGroupTargetFailoverElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbTargetGroupTargetFailoverElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbTargetGroupTargetFailoverElRef {
        LbTargetGroupTargetFailoverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbTargetGroupTargetFailoverElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_deregistration` after provisioning.\n"]
    pub fn on_deregistration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_deregistration", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unhealthy` after provisioning.\n"]
    pub fn on_unhealthy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_unhealthy", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbTargetGroupDynamic {
    health_check: std::option::Option<terrars::DynamicBlock<LbTargetGroupHealthCheckEl>>,
    stickiness: std::option::Option<terrars::DynamicBlock<LbTargetGroupStickinessEl>>,
    target_failover: std::option::Option<terrars::DynamicBlock<LbTargetGroupTargetFailoverEl>>,
}
