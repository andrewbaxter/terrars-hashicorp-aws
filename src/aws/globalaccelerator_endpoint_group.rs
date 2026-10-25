#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlobalacceleratorEndpointGroupData_ {
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
    endpoint_group_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listener_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_dial_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_configuration: std::option::Option<std::vec::Vec<GlobalacceleratorEndpointGroup_EndpointConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_override: std::option::Option<std::vec::Vec<GlobalacceleratorEndpointGroup_PortOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GlobalacceleratorEndpointGroup_Timeouts>,
    dynamic: GlobalacceleratorEndpointGroupDynamic,
}

struct GlobalacceleratorEndpointGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlobalacceleratorEndpointGroupData_>,
}

#[derive(Clone)]
pub struct GlobalacceleratorEndpointGroup(std::rc::Rc<GlobalacceleratorEndpointGroup_>);

impl GlobalacceleratorEndpointGroup {
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

    #[doc = "Set the field `endpoint_group_region`.\n"]
    pub fn set_endpoint_group_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint_group_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_interval_seconds`.\n"]
    pub fn set_health_check_interval_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_path`.\n"]
    pub fn set_health_check_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_port`.\n"]
    pub fn set_health_check_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_protocol`.\n"]
    pub fn set_health_check_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_count`.\n"]
    pub fn set_threshold_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_dial_percentage`.\n"]
    pub fn set_traffic_dial_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().traffic_dial_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlobalacceleratorEndpointGroup_EndpointConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_override`.\n"]
    pub fn set_port_override(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlobalacceleratorEndpointGroup_PortOverride>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().port_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.port_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GlobalacceleratorEndpointGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_interval_seconds` after provisioning.\n"]
    pub fn health_check_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_interval_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_port` after provisioning.\n"]
    pub fn health_check_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_protocol` after provisioning.\n"]
    pub fn health_check_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_count` after provisioning.\n"]
    pub fn threshold_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_dial_percentage` after provisioning.\n"]
    pub fn traffic_dial_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_dial_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorEndpointGroup_TimeoutsRef {
        GlobalacceleratorEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GlobalacceleratorEndpointGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlobalacceleratorEndpointGroup { }

impl terrars::ToListMappable for GlobalacceleratorEndpointGroup {
    type O = terrars::ListRef<GlobalacceleratorEndpointGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlobalacceleratorEndpointGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_globalaccelerator_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorEndpointGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub listener_arn: terrars::PrimField<std::string::String>,
}

impl BuildGlobalacceleratorEndpointGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> GlobalacceleratorEndpointGroup {
        let out = GlobalacceleratorEndpointGroup(std::rc::Rc::new(GlobalacceleratorEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlobalacceleratorEndpointGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint_group_region: core::default::Default::default(),
                health_check_interval_seconds: core::default::Default::default(),
                health_check_path: core::default::Default::default(),
                health_check_port: core::default::Default::default(),
                health_check_protocol: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                threshold_count: core::default::Default::default(),
                traffic_dial_percentage: core::default::Default::default(),
                endpoint_configuration: core::default::Default::default(),
                port_override: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorEndpointGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorEndpointGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlobalacceleratorEndpointGroupRef {
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

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_interval_seconds` after provisioning.\n"]
    pub fn health_check_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_interval_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_port` after provisioning.\n"]
    pub fn health_check_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_protocol` after provisioning.\n"]
    pub fn health_check_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_count` after provisioning.\n"]
    pub fn threshold_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_dial_percentage` after provisioning.\n"]
    pub fn traffic_dial_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_dial_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorEndpointGroup_TimeoutsRef {
        GlobalacceleratorEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct GlobalacceleratorEndpointGroupDynamic {
    endpoint_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlobalacceleratorEndpointGroup_EndpointConfiguration>>,
    port_override: std::option::Option<terrars::DynamicBlock<GlobalacceleratorEndpointGroup_PortOverride>>,
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroup_EndpointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ip_preservation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl GlobalacceleratorEndpointGroup_EndpointConfiguration {
    #[doc = "Set the field `attachment_arn`.\n"]
    pub fn set_attachment_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ip_preservation_enabled`.\n"]
    pub fn set_client_ip_preservation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_ip_preservation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorEndpointGroup_EndpointConfiguration {
    type O = terrars::BlockAssignable<GlobalacceleratorEndpointGroup_EndpointConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroup_EndpointConfiguration {}

impl BuildGlobalacceleratorEndpointGroup_EndpointConfiguration {
    pub fn build(self) -> GlobalacceleratorEndpointGroup_EndpointConfiguration {
        GlobalacceleratorEndpointGroup_EndpointConfiguration {
            attachment_arn: core::default::Default::default(),
            client_ip_preservation_enabled: core::default::Default::default(),
            endpoint_id: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorEndpointGroup_EndpointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorEndpointGroup_EndpointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlobalacceleratorEndpointGroup_EndpointConfigurationRef {
        GlobalacceleratorEndpointGroup_EndpointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroup_EndpointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_arn` after provisioning.\n"]
    pub fn attachment_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `client_ip_preservation_enabled` after provisioning.\n"]
    pub fn client_ip_preservation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ip_preservation_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroup_PortOverride {
    endpoint_port: terrars::PrimField<f64>,
    listener_port: terrars::PrimField<f64>,
}

impl GlobalacceleratorEndpointGroup_PortOverride { }

impl terrars::ToListMappable for GlobalacceleratorEndpointGroup_PortOverride {
    type O = terrars::BlockAssignable<GlobalacceleratorEndpointGroup_PortOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroup_PortOverride {
    #[doc = ""]
    pub endpoint_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub listener_port: terrars::PrimField<f64>,
}

impl BuildGlobalacceleratorEndpointGroup_PortOverride {
    pub fn build(self) -> GlobalacceleratorEndpointGroup_PortOverride {
        GlobalacceleratorEndpointGroup_PortOverride {
            endpoint_port: self.endpoint_port,
            listener_port: self.listener_port,
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorEndpointGroup_PortOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorEndpointGroup_PortOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlobalacceleratorEndpointGroup_PortOverrideRef {
        GlobalacceleratorEndpointGroup_PortOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroup_PortOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_port` after provisioning.\n"]
    pub fn endpoint_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_port", self.base))
    }

    #[doc = "Get a reference to the value of field `listener_port` after provisioning.\n"]
    pub fn listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorEndpointGroup_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorEndpointGroup_Timeouts {
    type O = terrars::BlockAssignable<GlobalacceleratorEndpointGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroup_Timeouts {}

impl BuildGlobalacceleratorEndpointGroup_Timeouts {
    pub fn build(self) -> GlobalacceleratorEndpointGroup_Timeouts {
        GlobalacceleratorEndpointGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorEndpointGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorEndpointGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlobalacceleratorEndpointGroup_TimeoutsRef {
        GlobalacceleratorEndpointGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroup_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
