use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53RecordData {
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
    allow_overwrite: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multivalue_answer_routing_policy: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    records: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::vec::Vec<Route53RecordAliasEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_routing_policy: std::option::Option<std::vec::Vec<Route53RecordFailoverRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geolocation_routing_policy: std::option::Option<std::vec::Vec<Route53RecordGeolocationRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latency_routing_policy: std::option::Option<std::vec::Vec<Route53RecordLatencyRoutingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_routing_policy: std::option::Option<std::vec::Vec<Route53RecordWeightedRoutingPolicyEl>>,
    dynamic: Route53RecordDynamic,
}

struct Route53Record_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53RecordData>,
}

#[derive(Clone)]
pub struct Route53Record(std::rc::Rc<Route53Record_>);

impl Route53Record {
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

    #[doc = "Set the field `allow_overwrite`.\n"]
    pub fn set_allow_overwrite(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_overwrite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_id`.\n"]
    pub fn set_health_check_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multivalue_answer_routing_policy`.\n"]
    pub fn set_multivalue_answer_routing_policy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multivalue_answer_routing_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `records`.\n"]
    pub fn set_records(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `set_identifier`.\n"]
    pub fn set_set_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().set_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordAliasEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alias = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alias = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failover_routing_policy`.\n"]
    pub fn set_failover_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordFailoverRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().failover_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.failover_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geolocation_routing_policy`.\n"]
    pub fn set_geolocation_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordGeolocationRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().geolocation_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.geolocation_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `latency_routing_policy`.\n"]
    pub fn set_latency_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordLatencyRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().latency_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.latency_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_routing_policy`.\n"]
    pub fn set_weighted_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordWeightedRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().weighted_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.weighted_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `allow_overwrite` after provisioning.\n"]
    pub fn allow_overwrite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multivalue_answer_routing_policy` after provisioning.\n"]
    pub fn multivalue_answer_routing_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.multivalue_answer_routing_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::ListRef<Route53RecordAliasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> terrars::ListRef<Route53RecordFailoverRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> terrars::ListRef<Route53RecordGeolocationRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> terrars::ListRef<Route53RecordLatencyRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> terrars::ListRef<Route53RecordWeightedRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_routing_policy", self.extract_ref()))
    }
}

impl terrars::Referable for Route53Record {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53Record { }

impl terrars::ToListMappable for Route53Record {
    type O = terrars::ListRef<Route53RecordRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53Record_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53_record".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53Record {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53Record {
        let out = Route53Record(std::rc::Rc::new(Route53Record_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53RecordData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_overwrite: core::default::Default::default(),
                health_check_id: core::default::Default::default(),
                id: core::default::Default::default(),
                multivalue_answer_routing_policy: core::default::Default::default(),
                name: self.name,
                records: core::default::Default::default(),
                set_identifier: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                zone_id: self.zone_id,
                alias: core::default::Default::default(),
                failover_routing_policy: core::default::Default::default(),
                geolocation_routing_policy: core::default::Default::default(),
                latency_routing_policy: core::default::Default::default(),
                weighted_routing_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53RecordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53RecordRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_overwrite` after provisioning.\n"]
    pub fn allow_overwrite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_overwrite", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\n"]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multivalue_answer_routing_policy` after provisioning.\n"]
    pub fn multivalue_answer_routing_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.multivalue_answer_routing_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `records` after provisioning.\n"]
    pub fn records(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::ListRef<Route53RecordAliasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> terrars::ListRef<Route53RecordFailoverRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> terrars::ListRef<Route53RecordGeolocationRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> terrars::ListRef<Route53RecordLatencyRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> terrars::ListRef<Route53RecordWeightedRoutingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_routing_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Route53RecordAliasEl {
    evaluate_target_health: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

impl Route53RecordAliasEl { }

impl terrars::ToListMappable for Route53RecordAliasEl {
    type O = terrars::BlockAssignable<Route53RecordAliasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordAliasEl {
    #[doc = ""]
    pub evaluate_target_health: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordAliasEl {
    pub fn build(self) -> Route53RecordAliasEl {
        Route53RecordAliasEl {
            evaluate_target_health: self.evaluate_target_health,
            name: self.name,
            zone_id: self.zone_id,
        }
    }
}

pub struct Route53RecordAliasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordAliasElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordAliasElRef {
        Route53RecordAliasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordAliasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordFailoverRoutingPolicyEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Route53RecordFailoverRoutingPolicyEl { }

impl terrars::ToListMappable for Route53RecordFailoverRoutingPolicyEl {
    type O = terrars::BlockAssignable<Route53RecordFailoverRoutingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordFailoverRoutingPolicyEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordFailoverRoutingPolicyEl {
    pub fn build(self) -> Route53RecordFailoverRoutingPolicyEl {
        Route53RecordFailoverRoutingPolicyEl { type_: self.type_ }
    }
}

pub struct Route53RecordFailoverRoutingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordFailoverRoutingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordFailoverRoutingPolicyElRef {
        Route53RecordFailoverRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordFailoverRoutingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordGeolocationRoutingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdivision: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53RecordGeolocationRoutingPolicyEl {
    #[doc = "Set the field `continent`.\n"]
    pub fn set_continent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.continent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdivision`.\n"]
    pub fn set_subdivision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subdivision = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53RecordGeolocationRoutingPolicyEl {
    type O = terrars::BlockAssignable<Route53RecordGeolocationRoutingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordGeolocationRoutingPolicyEl {}

impl BuildRoute53RecordGeolocationRoutingPolicyEl {
    pub fn build(self) -> Route53RecordGeolocationRoutingPolicyEl {
        Route53RecordGeolocationRoutingPolicyEl {
            continent: core::default::Default::default(),
            country: core::default::Default::default(),
            subdivision: core::default::Default::default(),
        }
    }
}

pub struct Route53RecordGeolocationRoutingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordGeolocationRoutingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordGeolocationRoutingPolicyElRef {
        Route53RecordGeolocationRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordGeolocationRoutingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `continent` after provisioning.\n"]
    pub fn continent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continent", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `subdivision` after provisioning.\n"]
    pub fn subdivision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdivision", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordLatencyRoutingPolicyEl {
    region: terrars::PrimField<std::string::String>,
}

impl Route53RecordLatencyRoutingPolicyEl { }

impl terrars::ToListMappable for Route53RecordLatencyRoutingPolicyEl {
    type O = terrars::BlockAssignable<Route53RecordLatencyRoutingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordLatencyRoutingPolicyEl {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordLatencyRoutingPolicyEl {
    pub fn build(self) -> Route53RecordLatencyRoutingPolicyEl {
        Route53RecordLatencyRoutingPolicyEl { region: self.region }
    }
}

pub struct Route53RecordLatencyRoutingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordLatencyRoutingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordLatencyRoutingPolicyElRef {
        Route53RecordLatencyRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordLatencyRoutingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordWeightedRoutingPolicyEl {
    weight: terrars::PrimField<f64>,
}

impl Route53RecordWeightedRoutingPolicyEl { }

impl terrars::ToListMappable for Route53RecordWeightedRoutingPolicyEl {
    type O = terrars::BlockAssignable<Route53RecordWeightedRoutingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordWeightedRoutingPolicyEl {
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildRoute53RecordWeightedRoutingPolicyEl {
    pub fn build(self) -> Route53RecordWeightedRoutingPolicyEl {
        Route53RecordWeightedRoutingPolicyEl { weight: self.weight }
    }
}

pub struct Route53RecordWeightedRoutingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordWeightedRoutingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordWeightedRoutingPolicyElRef {
        Route53RecordWeightedRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordWeightedRoutingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordDynamic {
    alias: std::option::Option<terrars::DynamicBlock<Route53RecordAliasEl>>,
    failover_routing_policy: std::option::Option<terrars::DynamicBlock<Route53RecordFailoverRoutingPolicyEl>>,
    geolocation_routing_policy: std::option::Option<terrars::DynamicBlock<Route53RecordGeolocationRoutingPolicyEl>>,
    latency_routing_policy: std::option::Option<terrars::DynamicBlock<Route53RecordLatencyRoutingPolicyEl>>,
    weighted_routing_policy: std::option::Option<terrars::DynamicBlock<Route53RecordWeightedRoutingPolicyEl>>,
}
