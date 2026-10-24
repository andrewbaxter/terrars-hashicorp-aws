use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ServiceDiscoveryServiceData {
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
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_config: std::option::Option<std::vec::Vec<ServiceDiscoveryServiceDnsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_config: std::option::Option<std::vec::Vec<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_custom_config: std::option::Option<std::vec::Vec<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
    dynamic: ServiceDiscoveryServiceDynamic,
}

struct ServiceDiscoveryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ServiceDiscoveryServiceData>,
}

#[derive(Clone)]
pub struct ServiceDiscoveryService(std::rc::Rc<ServiceDiscoveryService_>);

impl ServiceDiscoveryService {
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

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().namespace_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_config`.\n"]
    pub fn set_dns_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceDiscoveryServiceDnsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check_config`.\n"]
    pub fn set_health_check_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check_custom_config`.\n"]
    pub fn set_health_check_custom_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_custom_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_custom_config = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceHealthCheckConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }
}

impl terrars::Referable for ServiceDiscoveryService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ServiceDiscoveryService { }

impl terrars::ToListMappable for ServiceDiscoveryService {
    type O = terrars::ListRef<ServiceDiscoveryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ServiceDiscoveryService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_service_discovery_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServiceDiscoveryService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildServiceDiscoveryService {
    pub fn build(self, stack: &mut terrars::Stack) -> ServiceDiscoveryService {
        let out = ServiceDiscoveryService(std::rc::Rc::new(ServiceDiscoveryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ServiceDiscoveryServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                namespace_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                dns_config: core::default::Default::default(),
                health_check_config: core::default::Default::default(),
                health_check_custom_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceDiscoveryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceDiscoveryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceDiscoveryServiceRef {
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

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceHealthCheckConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> terrars::ListRef<ServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    ttl: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ServiceDiscoveryServiceDnsConfigElDnsRecordsEl { }

impl terrars::ToListMappable for ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    type O = terrars::BlockAssignable<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[doc = ""]
    pub ttl: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    pub fn build(self) -> ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
        ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
            ttl: self.ttl,
            type_: self.type_,
        }
    }
}

pub struct ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
        ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceDiscoveryServiceDnsConfigElDynamic {
    dns_records: std::option::Option<terrars::DynamicBlock<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceDnsConfigEl {
    namespace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_records: std::option::Option<std::vec::Vec<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    dynamic: ServiceDiscoveryServiceDnsConfigElDynamic,
}

impl ServiceDiscoveryServiceDnsConfigEl {
    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_records`.\n"]
    pub fn set_dns_records(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_records = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_records = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ServiceDiscoveryServiceDnsConfigEl {
    type O = terrars::BlockAssignable<ServiceDiscoveryServiceDnsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceDnsConfigEl {
    #[doc = ""]
    pub namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildServiceDiscoveryServiceDnsConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceDnsConfigEl {
        ServiceDiscoveryServiceDnsConfigEl {
            namespace_id: self.namespace_id,
            routing_policy: core::default::Default::default(),
            dns_records: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceDiscoveryServiceDnsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceDiscoveryServiceDnsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceDiscoveryServiceDnsConfigElRef {
        ServiceDiscoveryServiceDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceDnsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> terrars::ListRef<ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceHealthCheckConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceDiscoveryServiceHealthCheckConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_path`.\n"]
    pub fn set_resource_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceDiscoveryServiceHealthCheckConfigEl {
    type O = terrars::BlockAssignable<ServiceDiscoveryServiceHealthCheckConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceHealthCheckConfigEl {}

impl BuildServiceDiscoveryServiceHealthCheckConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceHealthCheckConfigEl {
        ServiceDiscoveryServiceHealthCheckConfigEl {
            failure_threshold: core::default::Default::default(),
            resource_path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ServiceDiscoveryServiceHealthCheckConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceDiscoveryServiceHealthCheckConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceDiscoveryServiceHealthCheckConfigElRef {
        ServiceDiscoveryServiceHealthCheckConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceHealthCheckConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_path", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    type O = terrars::BlockAssignable<ServiceDiscoveryServiceHealthCheckCustomConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceHealthCheckCustomConfigEl {}

impl BuildServiceDiscoveryServiceHealthCheckCustomConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceHealthCheckCustomConfigEl {
        ServiceDiscoveryServiceHealthCheckCustomConfigEl { failure_threshold: core::default::Default::default() }
    }
}

pub struct ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
        ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceDiscoveryServiceDynamic {
    dns_config: std::option::Option<terrars::DynamicBlock<ServiceDiscoveryServiceDnsConfigEl>>,
    health_check_config: std::option::Option<terrars::DynamicBlock<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    health_check_custom_config: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
}
