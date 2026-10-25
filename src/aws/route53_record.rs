#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53RecordData_ {
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
    alias: std::option::Option<std::vec::Vec<Route53Record_Alias>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_routing_policy: std::option::Option<std::vec::Vec<Route53Record_CidrRoutingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_routing_policy: std::option::Option<std::vec::Vec<Route53Record_FailoverRoutingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geolocation_routing_policy: std::option::Option<std::vec::Vec<Route53Record_GeolocationRoutingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geoproximity_routing_policy: std::option::Option<std::vec::Vec<Route53Record_GeoproximityRoutingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latency_routing_policy: std::option::Option<std::vec::Vec<Route53Record_LatencyRoutingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Route53Record_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_routing_policy: std::option::Option<std::vec::Vec<Route53Record_WeightedRoutingPolicy>>,
    dynamic: Route53RecordDynamic,
}

struct Route53Record_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53RecordData_>,
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
    pub fn set_alias(self, v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_Alias>>) -> Self {
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

    #[doc = "Set the field `cidr_routing_policy`.\n"]
    pub fn set_cidr_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_CidrRoutingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failover_routing_policy`.\n"]
    pub fn set_failover_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_FailoverRoutingPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_GeolocationRoutingPolicy>>,
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

    #[doc = "Set the field `geoproximity_routing_policy`.\n"]
    pub fn set_geoproximity_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_GeoproximityRoutingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().geoproximity_routing_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.geoproximity_routing_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `latency_routing_policy`.\n"]
    pub fn set_latency_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_LatencyRoutingPolicy>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Route53Record_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_routing_policy`.\n"]
    pub fn set_weighted_routing_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_WeightedRoutingPolicy>>,
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
    pub fn alias(&self) -> terrars::ListRef<Route53Record_AliasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_policy` after provisioning.\n"]
    pub fn cidr_routing_policy(&self) -> terrars::ListRef<Route53Record_CidrRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> terrars::ListRef<Route53Record_FailoverRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> terrars::ListRef<Route53Record_GeolocationRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geoproximity_routing_policy` after provisioning.\n"]
    pub fn geoproximity_routing_policy(&self) -> terrars::ListRef<Route53Record_GeoproximityRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geoproximity_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> terrars::ListRef<Route53Record_LatencyRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53Record_TimeoutsRef {
        Route53Record_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> terrars::ListRef<Route53Record_WeightedRoutingPolicyRef> {
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
            data: std::cell::RefCell::new(Route53RecordData_ {
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
                cidr_routing_policy: core::default::Default::default(),
                failover_routing_policy: core::default::Default::default(),
                geolocation_routing_policy: core::default::Default::default(),
                geoproximity_routing_policy: core::default::Default::default(),
                latency_routing_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
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
    pub fn alias(&self) -> terrars::ListRef<Route53Record_AliasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_policy` after provisioning.\n"]
    pub fn cidr_routing_policy(&self) -> terrars::ListRef<Route53Record_CidrRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failover_routing_policy` after provisioning.\n"]
    pub fn failover_routing_policy(&self) -> terrars::ListRef<Route53Record_FailoverRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geolocation_routing_policy` after provisioning.\n"]
    pub fn geolocation_routing_policy(&self) -> terrars::ListRef<Route53Record_GeolocationRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geolocation_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `geoproximity_routing_policy` after provisioning.\n"]
    pub fn geoproximity_routing_policy(&self) -> terrars::ListRef<Route53Record_GeoproximityRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geoproximity_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latency_routing_policy` after provisioning.\n"]
    pub fn latency_routing_policy(&self) -> terrars::ListRef<Route53Record_LatencyRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency_routing_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53Record_TimeoutsRef {
        Route53Record_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weighted_routing_policy` after provisioning.\n"]
    pub fn weighted_routing_policy(&self) -> terrars::ListRef<Route53Record_WeightedRoutingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_routing_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordDynamic {
    alias: std::option::Option<terrars::DynamicBlock<Route53Record_Alias>>,
    cidr_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_CidrRoutingPolicy>>,
    failover_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_FailoverRoutingPolicy>>,
    geolocation_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_GeolocationRoutingPolicy>>,
    geoproximity_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_GeoproximityRoutingPolicy>>,
    latency_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_LatencyRoutingPolicy>>,
    weighted_routing_policy: std::option::Option<terrars::DynamicBlock<Route53Record_WeightedRoutingPolicy>>,
}

#[derive(Serialize)]
pub struct Route53Record_Alias {
    evaluate_target_health: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

impl Route53Record_Alias { }

impl terrars::ToListMappable for Route53Record_Alias {
    type O = terrars::BlockAssignable<Route53Record_Alias>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_Alias {
    #[doc = ""]
    pub evaluate_target_health: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record_Alias {
    pub fn build(self) -> Route53Record_Alias {
        Route53Record_Alias {
            evaluate_target_health: self.evaluate_target_health,
            name: self.name,
            zone_id: self.zone_id,
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_AliasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_AliasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_AliasRef {
        Route53Record_AliasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_AliasRef {
    #[allow(dead_code)]
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
pub struct Route53Record_CidrRoutingPolicy {
    collection_id: terrars::PrimField<std::string::String>,
    location_name: terrars::PrimField<std::string::String>,
}

impl Route53Record_CidrRoutingPolicy { }

impl terrars::ToListMappable for Route53Record_CidrRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_CidrRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_CidrRoutingPolicy {
    #[doc = ""]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location_name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record_CidrRoutingPolicy {
    pub fn build(self) -> Route53Record_CidrRoutingPolicy {
        Route53Record_CidrRoutingPolicy {
            collection_id: self.collection_id,
            location_name: self.location_name,
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_CidrRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_CidrRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_CidrRoutingPolicyRef {
        Route53Record_CidrRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_CidrRoutingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\n"]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `location_name` after provisioning.\n"]
    pub fn location_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53Record_FailoverRoutingPolicy {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Route53Record_FailoverRoutingPolicy { }

impl terrars::ToListMappable for Route53Record_FailoverRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_FailoverRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_FailoverRoutingPolicy {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record_FailoverRoutingPolicy {
    pub fn build(self) -> Route53Record_FailoverRoutingPolicy {
        Route53Record_FailoverRoutingPolicy { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct Route53Record_FailoverRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_FailoverRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_FailoverRoutingPolicyRef {
        Route53Record_FailoverRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_FailoverRoutingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53Record_GeolocationRoutingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdivision: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53Record_GeolocationRoutingPolicy {
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

impl terrars::ToListMappable for Route53Record_GeolocationRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_GeolocationRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_GeolocationRoutingPolicy {}

impl BuildRoute53Record_GeolocationRoutingPolicy {
    pub fn build(self) -> Route53Record_GeolocationRoutingPolicy {
        Route53Record_GeolocationRoutingPolicy {
            continent: core::default::Default::default(),
            country: core::default::Default::default(),
            subdivision: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_GeolocationRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_GeolocationRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_GeolocationRoutingPolicyRef {
        Route53Record_GeolocationRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_GeolocationRoutingPolicyRef {
    #[allow(dead_code)]
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
pub struct Route53Record_GeoproximityRoutingPolicy_Coordinates {
    latitude: terrars::PrimField<std::string::String>,
    longitude: terrars::PrimField<std::string::String>,
}

impl Route53Record_GeoproximityRoutingPolicy_Coordinates { }

impl terrars::ToListMappable for Route53Record_GeoproximityRoutingPolicy_Coordinates {
    type O = terrars::BlockAssignable<Route53Record_GeoproximityRoutingPolicy_Coordinates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_GeoproximityRoutingPolicy_Coordinates {
    #[doc = ""]
    pub latitude: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub longitude: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record_GeoproximityRoutingPolicy_Coordinates {
    pub fn build(self) -> Route53Record_GeoproximityRoutingPolicy_Coordinates {
        Route53Record_GeoproximityRoutingPolicy_Coordinates {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_GeoproximityRoutingPolicy_CoordinatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_GeoproximityRoutingPolicy_CoordinatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53Record_GeoproximityRoutingPolicy_CoordinatesRef {
        Route53Record_GeoproximityRoutingPolicy_CoordinatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_GeoproximityRoutingPolicy_CoordinatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latitude` after provisioning.\n"]
    pub fn latitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc = "Get a reference to the value of field `longitude` after provisioning.\n"]
    pub fn longitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53Record_GeoproximityRoutingPolicyDynamic {
    coordinates: std::option::Option<terrars::DynamicBlock<Route53Record_GeoproximityRoutingPolicy_Coordinates>>,
}

#[derive(Serialize)]
pub struct Route53Record_GeoproximityRoutingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bias: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_zone_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coordinates: std::option::Option<std::vec::Vec<Route53Record_GeoproximityRoutingPolicy_Coordinates>>,
    dynamic: Route53Record_GeoproximityRoutingPolicyDynamic,
}

impl Route53Record_GeoproximityRoutingPolicy {
    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bias`.\n"]
    pub fn set_bias(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_zone_group`.\n"]
    pub fn set_local_zone_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_zone_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coordinates`.\n"]
    pub fn set_coordinates(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53Record_GeoproximityRoutingPolicy_Coordinates>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.coordinates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.coordinates = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53Record_GeoproximityRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_GeoproximityRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_GeoproximityRoutingPolicy {}

impl BuildRoute53Record_GeoproximityRoutingPolicy {
    pub fn build(self) -> Route53Record_GeoproximityRoutingPolicy {
        Route53Record_GeoproximityRoutingPolicy {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_GeoproximityRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_GeoproximityRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_GeoproximityRoutingPolicyRef {
        Route53Record_GeoproximityRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_GeoproximityRoutingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `bias` after provisioning.\n"]
    pub fn bias(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bias", self.base))
    }

    #[doc = "Get a reference to the value of field `local_zone_group` after provisioning.\n"]
    pub fn local_zone_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_zone_group", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53Record_LatencyRoutingPolicy {
    region: terrars::PrimField<std::string::String>,
}

impl Route53Record_LatencyRoutingPolicy { }

impl terrars::ToListMappable for Route53Record_LatencyRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_LatencyRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_LatencyRoutingPolicy {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildRoute53Record_LatencyRoutingPolicy {
    pub fn build(self) -> Route53Record_LatencyRoutingPolicy {
        Route53Record_LatencyRoutingPolicy { region: self.region }
    }
}

#[allow(dead_code)]
pub struct Route53Record_LatencyRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_LatencyRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_LatencyRoutingPolicyRef {
        Route53Record_LatencyRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_LatencyRoutingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53Record_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53Record_Timeouts {
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

impl terrars::ToListMappable for Route53Record_Timeouts {
    type O = terrars::BlockAssignable<Route53Record_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_Timeouts {}

impl BuildRoute53Record_Timeouts {
    pub fn build(self) -> Route53Record_Timeouts {
        Route53Record_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53Record_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_TimeoutsRef {
        Route53Record_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_TimeoutsRef {
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

#[derive(Serialize)]
pub struct Route53Record_WeightedRoutingPolicy {
    weight: terrars::PrimField<f64>,
}

impl Route53Record_WeightedRoutingPolicy { }

impl terrars::ToListMappable for Route53Record_WeightedRoutingPolicy {
    type O = terrars::BlockAssignable<Route53Record_WeightedRoutingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53Record_WeightedRoutingPolicy {
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildRoute53Record_WeightedRoutingPolicy {
    pub fn build(self) -> Route53Record_WeightedRoutingPolicy {
        Route53Record_WeightedRoutingPolicy { weight: self.weight }
    }
}

#[allow(dead_code)]
pub struct Route53Record_WeightedRoutingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53Record_WeightedRoutingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53Record_WeightedRoutingPolicyRef {
        Route53Record_WeightedRoutingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53Record_WeightedRoutingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
