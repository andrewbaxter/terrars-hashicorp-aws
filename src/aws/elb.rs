use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElbData {
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
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_draining: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_draining_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_zone_load_balancing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desync_mitigation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_security_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_logs: std::option::Option<std::vec::Vec<ElbAccessLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<ElbHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<std::vec::Vec<ElbListenerEl>>,
    dynamic: ElbDynamic,
}

struct Elb_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElbData>,
}

#[derive(Clone)]
pub struct Elb(std::rc::Rc<Elb_>);

impl Elb {
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

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_draining`.\n"]
    pub fn set_connection_draining(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connection_draining = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_draining_timeout`.\n"]
    pub fn set_connection_draining_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connection_draining_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cross_zone_load_balancing`.\n"]
    pub fn set_cross_zone_load_balancing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cross_zone_load_balancing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desync_mitigation_mode`.\n"]
    pub fn set_desync_mitigation_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().desync_mitigation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal`.\n"]
    pub fn set_internal(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_security_group`.\n"]
    pub fn set_source_security_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnets = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(self, v: impl std::convert::Into<terrars::BlockAssignable<ElbAccessLogsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(self, v: impl std::convert::Into<terrars::BlockAssignable<ElbHealthCheckEl>>) -> Self {
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

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(self, v: impl std::convert::Into<terrars::BlockAssignable<ElbListenerEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().listener = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.listener = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<ElbAccessLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<ElbHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }
}

impl terrars::Referable for Elb {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Elb { }

impl terrars::ToListMappable for Elb {
    type O = terrars::ListRef<ElbRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Elb_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elb".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElb {
    pub tf_id: std::string::String,
}

impl BuildElb {
    pub fn build(self, stack: &mut terrars::Stack) -> Elb {
        let out = Elb(std::rc::Rc::new(Elb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElbData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zones: core::default::Default::default(),
                connection_draining: core::default::Default::default(),
                connection_draining_timeout: core::default::Default::default(),
                cross_zone_load_balancing: core::default::Default::default(),
                desync_mitigation_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                instances: core::default::Default::default(),
                internal: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_security_group: core::default::Default::default(),
                subnets: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_logs: core::default::Default::default(),
                health_check: core::default::Default::default(),
                listener: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElbRef {
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<ElbAccessLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<ElbHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ElbAccessLogsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
}

impl ElbAccessLogsEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElbAccessLogsEl {
    type O = terrars::BlockAssignable<ElbAccessLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElbAccessLogsEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildElbAccessLogsEl {
    pub fn build(self) -> ElbAccessLogsEl {
        ElbAccessLogsEl {
            bucket: self.bucket,
            bucket_prefix: core::default::Default::default(),
            enabled: core::default::Default::default(),
            interval: core::default::Default::default(),
        }
    }
}

pub struct ElbAccessLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElbAccessLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElbAccessLogsElRef {
        ElbAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElbAccessLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct ElbHealthCheckEl {
    healthy_threshold: terrars::PrimField<f64>,
    interval: terrars::PrimField<f64>,
    target: terrars::PrimField<std::string::String>,
    timeout: terrars::PrimField<f64>,
    unhealthy_threshold: terrars::PrimField<f64>,
}

impl ElbHealthCheckEl { }

impl terrars::ToListMappable for ElbHealthCheckEl {
    type O = terrars::BlockAssignable<ElbHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElbHealthCheckEl {
    #[doc = ""]
    pub healthy_threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout: terrars::PrimField<f64>,
    #[doc = ""]
    pub unhealthy_threshold: terrars::PrimField<f64>,
}

impl BuildElbHealthCheckEl {
    pub fn build(self) -> ElbHealthCheckEl {
        ElbHealthCheckEl {
            healthy_threshold: self.healthy_threshold,
            interval: self.interval,
            target: self.target,
            timeout: self.timeout,
            unhealthy_threshold: self.unhealthy_threshold,
        }
    }
}

pub struct ElbHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElbHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElbHealthCheckElRef {
        ElbHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElbHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
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
pub struct ElbListenerEl {
    instance_port: terrars::PrimField<f64>,
    instance_protocol: terrars::PrimField<std::string::String>,
    lb_port: terrars::PrimField<f64>,
    lb_protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElbListenerEl {
    #[doc = "Set the field `ssl_certificate_id`.\n"]
    pub fn set_ssl_certificate_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_certificate_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElbListenerEl {
    type O = terrars::BlockAssignable<ElbListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElbListenerEl {
    #[doc = ""]
    pub instance_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lb_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub lb_protocol: terrars::PrimField<std::string::String>,
}

impl BuildElbListenerEl {
    pub fn build(self) -> ElbListenerEl {
        ElbListenerEl {
            instance_port: self.instance_port,
            instance_protocol: self.instance_protocol,
            lb_port: self.lb_port,
            lb_protocol: self.lb_protocol,
            ssl_certificate_id: core::default::Default::default(),
        }
    }
}

pub struct ElbListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElbListenerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElbListenerElRef {
        ElbListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElbListenerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_port` after provisioning.\n"]
    pub fn instance_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_port", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_protocol` after provisioning.\n"]
    pub fn instance_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `lb_port` after provisioning.\n"]
    pub fn lb_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lb_port", self.base))
    }

    #[doc = "Get a reference to the value of field `lb_protocol` after provisioning.\n"]
    pub fn lb_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lb_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_certificate_id` after provisioning.\n"]
    pub fn ssl_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_certificate_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElbDynamic {
    access_logs: std::option::Option<terrars::DynamicBlock<ElbAccessLogsEl>>,
    health_check: std::option::Option<terrars::DynamicBlock<ElbHealthCheckEl>>,
    listener: std::option::Option<terrars::DynamicBlock<ElbListenerEl>>,
}
