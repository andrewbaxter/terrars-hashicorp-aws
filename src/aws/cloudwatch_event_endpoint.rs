#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchEventEndpointData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_bus: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_EventBus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_config: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_ReplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_config: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_RoutingConfig>>,
    dynamic: CloudwatchEventEndpointDynamic,
}

struct CloudwatchEventEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchEventEndpointData_>,
}

#[derive(Clone)]
pub struct CloudwatchEventEndpoint(std::rc::Rc<CloudwatchEventEndpoint_>);

impl CloudwatchEventEndpoint {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_bus`.\n"]
    pub fn set_event_bus(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_EventBus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_bus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_bus = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_config`.\n"]
    pub fn set_replication_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_ReplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bus` after provisioning.\n"]
    pub fn event_bus(&self) -> terrars::ListRef<CloudwatchEventEndpoint_EventBusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_config` after provisioning.\n"]
    pub fn replication_config(&self) -> terrars::ListRef<CloudwatchEventEndpoint_ReplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> terrars::ListRef<CloudwatchEventEndpoint_RoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchEventEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchEventEndpoint { }

impl terrars::ToListMappable for CloudwatchEventEndpoint {
    type O = terrars::ListRef<CloudwatchEventEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchEventEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_event_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchEventEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchEventEndpoint {
        let out = CloudwatchEventEndpoint(std::rc::Rc::new(CloudwatchEventEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchEventEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                event_bus: core::default::Default::default(),
                replication_config: core::default::Default::default(),
                routing_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchEventEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchEventEndpointRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bus` after provisioning.\n"]
    pub fn event_bus(&self) -> terrars::ListRef<CloudwatchEventEndpoint_EventBusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_config` after provisioning.\n"]
    pub fn replication_config(&self) -> terrars::ListRef<CloudwatchEventEndpoint_ReplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> terrars::ListRef<CloudwatchEventEndpoint_RoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpointDynamic {
    event_bus: std::option::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_EventBus>>,
    replication_config: std::option::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_ReplicationConfig>>,
    routing_config: std::option::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_RoutingConfig>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_EventBus {
    event_bus_arn: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventEndpoint_EventBus { }

impl terrars::ToListMappable for CloudwatchEventEndpoint_EventBus {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_EventBus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_EventBus {
    #[doc = ""]
    pub event_bus_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventEndpoint_EventBus {
    pub fn build(self) -> CloudwatchEventEndpoint_EventBus {
        CloudwatchEventEndpoint_EventBus { event_bus_arn: self.event_bus_arn }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_EventBusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_EventBusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventEndpoint_EventBusRef {
        CloudwatchEventEndpoint_EventBusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_EventBusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_bus_arn` after provisioning.\n"]
    pub fn event_bus_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_bus_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_ReplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventEndpoint_ReplicationConfig {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventEndpoint_ReplicationConfig {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_ReplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_ReplicationConfig {}

impl BuildCloudwatchEventEndpoint_ReplicationConfig {
    pub fn build(self) -> CloudwatchEventEndpoint_ReplicationConfig {
        CloudwatchEventEndpoint_ReplicationConfig { state: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_ReplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_ReplicationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventEndpoint_ReplicationConfigRef {
        CloudwatchEventEndpoint_ReplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_ReplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {}

impl BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
    pub fn build(self) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary {
            health_check: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {
    #[doc = "Set the field `route`.\n"]
    pub fn set_route(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {}

impl BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {
    pub fn build(self) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary { route: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfigDynamic {
    primary: std::option::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary>>,
    secondary: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary>>,
    dynamic: CloudwatchEventEndpoint_RoutingConfig_FailoverConfigDynamic,
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Primary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary`.\n"]
    pub fn set_secondary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_Secondary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secondary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig {}

impl BuildCloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
    pub fn build(self) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfig {
            primary: core::default::Default::default(),
            secondary: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef {
        CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::ListRef<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_PrimaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary` after provisioning.\n"]
    pub fn secondary(&self) -> terrars::ListRef<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig_SecondaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpoint_RoutingConfigDynamic {
    failover_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpoint_RoutingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_config: std::option::Option<std::vec::Vec<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig>>,
    dynamic: CloudwatchEventEndpoint_RoutingConfigDynamic,
}

impl CloudwatchEventEndpoint_RoutingConfig {
    #[doc = "Set the field `failover_config`.\n"]
    pub fn set_failover_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig_FailoverConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failover_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventEndpoint_RoutingConfig {
    type O = terrars::BlockAssignable<CloudwatchEventEndpoint_RoutingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpoint_RoutingConfig {}

impl BuildCloudwatchEventEndpoint_RoutingConfig {
    pub fn build(self) -> CloudwatchEventEndpoint_RoutingConfig {
        CloudwatchEventEndpoint_RoutingConfig {
            failover_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventEndpoint_RoutingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventEndpoint_RoutingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventEndpoint_RoutingConfigRef {
        CloudwatchEventEndpoint_RoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpoint_RoutingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover_config` after provisioning.\n"]
    pub fn failover_config(&self) -> terrars::ListRef<CloudwatchEventEndpoint_RoutingConfig_FailoverConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_config", self.base))
    }
}
