#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53RecordsExclusiveData_ {
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
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_record_set: std::option::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Route53RecordsExclusive_Timeouts>,
    dynamic: Route53RecordsExclusiveDynamic,
}

struct Route53RecordsExclusive_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53RecordsExclusiveData_>,
}

#[derive(Clone)]
pub struct Route53RecordsExclusive(std::rc::Rc<Route53RecordsExclusive_>);

impl Route53RecordsExclusive {
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

    #[doc = "Set the field `resource_record_set`.\n"]
    pub fn set_resource_record_set(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_record_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_record_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Route53RecordsExclusive_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordsExclusive_TimeoutsRef {
        Route53RecordsExclusive_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Route53RecordsExclusive {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53RecordsExclusive { }

impl terrars::ToListMappable for Route53RecordsExclusive {
    type O = terrars::ListRef<Route53RecordsExclusiveRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53RecordsExclusive_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53_records_exclusive".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53RecordsExclusive {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53RecordsExclusive {
        let out = Route53RecordsExclusive(std::rc::Rc::new(Route53RecordsExclusive_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53RecordsExclusiveData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                zone_id: self.zone_id,
                resource_record_set: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53RecordsExclusiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusiveRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53RecordsExclusiveRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53RecordsExclusive_TimeoutsRef {
        Route53RecordsExclusive_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordsExclusiveDynamic {
    resource_record_set: std::option::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet>>,
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet_AliasTarget {
    dns_name: terrars::PrimField<std::string::String>,
    evaluate_target_health: terrars::PrimField<bool>,
    hosted_zone_id: terrars::PrimField<std::string::String>,
}

impl Route53RecordsExclusive_ResourceRecordSet_AliasTarget { }

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_AliasTarget {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_AliasTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_AliasTarget {
    #[doc = ""]
    pub dns_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub evaluate_target_health: terrars::PrimField<bool>,
    #[doc = ""]
    pub hosted_zone_id: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_AliasTarget {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_AliasTarget {
        Route53RecordsExclusive_ResourceRecordSet_AliasTarget {
            dns_name: self.dns_name,
            evaluate_target_health: self.evaluate_target_health,
            hosted_zone_id: self.hosted_zone_id,
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef {
        Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
    collection_id: terrars::PrimField<std::string::String>,
    location_name: terrars::PrimField<std::string::String>,
}

impl Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig { }

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
    #[doc = ""]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location_name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
        Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig {
            collection_id: self.collection_id,
            location_name: self.location_name,
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef {
        Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef {
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
pub struct Route53RecordsExclusive_ResourceRecordSet_Geolocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continent_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdivision_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53RecordsExclusive_ResourceRecordSet_Geolocation {
    #[doc = "Set the field `continent_code`.\n"]
    pub fn set_continent_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.continent_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdivision_code`.\n"]
    pub fn set_subdivision_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subdivision_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_Geolocation {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_Geolocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_Geolocation {}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_Geolocation {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_Geolocation {
        Route53RecordsExclusive_ResourceRecordSet_Geolocation {
            continent_code: core::default::Default::default(),
            country_code: core::default::Default::default(),
            subdivision_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_GeolocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_GeolocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_GeolocationRef {
        Route53RecordsExclusive_ResourceRecordSet_GeolocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_GeolocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `continent_code` after provisioning.\n"]
    pub fn continent_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continent_code", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `subdivision_code` after provisioning.\n"]
    pub fn subdivision_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdivision_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
    latitude: terrars::PrimField<std::string::String>,
    longitude: terrars::PrimField<std::string::String>,
}

impl Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates { }

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
    #[doc = ""]
    pub latitude: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub longitude: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
        Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef {
        Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef {
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
struct Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationDynamic {
    coordinates: std
    ::option
    ::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates>>,
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bias: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_zone_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coordinates: std
    ::option
    ::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates>>,
    dynamic: Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationDynamic,
}

impl Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_Coordinates>,
                    >,
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

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
        Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef {
        Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef {
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

    #[doc = "Get a reference to the value of field `coordinates` after provisioning.\n"]
    pub fn coordinates(
        &self,
    ) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation_CoordinatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coordinates", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet_ResourceRecords {
    value: terrars::PrimField<std::string::String>,
}

impl Route53RecordsExclusive_ResourceRecordSet_ResourceRecords { }

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet_ResourceRecords {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_ResourceRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet_ResourceRecords {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive_ResourceRecordSet_ResourceRecords {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet_ResourceRecords {
        Route53RecordsExclusive_ResourceRecordSet_ResourceRecords { value: self.value }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef {
        Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53RecordsExclusive_ResourceRecordSetDynamic {
    alias_target: std::option::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_AliasTarget>>,
    cidr_routing_config: std
    ::option
    ::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig>>,
    geolocation: std::option::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_Geolocation>>,
    geoproximity_location: std
    ::option
    ::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation>>,
    resource_records: std
    ::option
    ::Option<terrars::DynamicBlock<Route53RecordsExclusive_ResourceRecordSet_ResourceRecords>>,
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_ResourceRecordSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_value_answer: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_policy_instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_target: std::option::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_AliasTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_routing_config: std::option::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geolocation: std::option::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_Geolocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geoproximity_location: std
    ::option
    ::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_records: std::option::Option<std::vec::Vec<Route53RecordsExclusive_ResourceRecordSet_ResourceRecords>>,
    dynamic: Route53RecordsExclusive_ResourceRecordSetDynamic,
}

impl Route53RecordsExclusive_ResourceRecordSet {
    #[doc = "Set the field `failover`.\n"]
    pub fn set_failover(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failover = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_id`.\n"]
    pub fn set_health_check_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_value_answer`.\n"]
    pub fn set_multi_value_answer(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.multi_value_answer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `set_identifier`.\n"]
    pub fn set_set_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.set_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_policy_instance_id`.\n"]
    pub fn set_traffic_policy_instance_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.traffic_policy_instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alias_target`.\n"]
    pub fn set_alias_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_AliasTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alias_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cidr_routing_config`.\n"]
    pub fn set_cidr_routing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cidr_routing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_routing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geolocation`.\n"]
    pub fn set_geolocation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_Geolocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geolocation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geolocation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geoproximity_location`.\n"]
    pub fn set_geoproximity_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geoproximity_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geoproximity_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_records`.\n"]
    pub fn set_resource_records(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet_ResourceRecords>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_records = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_records = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53RecordsExclusive_ResourceRecordSet {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_ResourceRecordSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_ResourceRecordSet {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53RecordsExclusive_ResourceRecordSet {
    pub fn build(self) -> Route53RecordsExclusive_ResourceRecordSet {
        Route53RecordsExclusive_ResourceRecordSet {
            failover: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            multi_value_answer: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            set_identifier: core::default::Default::default(),
            traffic_policy_instance_id: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
            weight: core::default::Default::default(),
            alias_target: core::default::Default::default(),
            cidr_routing_config: core::default::Default::default(),
            geolocation: core::default::Default::default(),
            geoproximity_location: core::default::Default::default(),
            resource_records: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_ResourceRecordSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_ResourceRecordSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordsExclusive_ResourceRecordSetRef {
        Route53RecordsExclusive_ResourceRecordSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_ResourceRecordSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover` after provisioning.\n"]
    pub fn failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_value_answer` after provisioning.\n"]
    pub fn multi_value_answer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_value_answer", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `set_identifier` after provisioning.\n"]
    pub fn set_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.set_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_policy_instance_id` after provisioning.\n"]
    pub fn traffic_policy_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_policy_instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `alias_target` after provisioning.\n"]
    pub fn alias_target(&self) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_AliasTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias_target", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_routing_config` after provisioning.\n"]
    pub fn cidr_routing_config(
        &self,
    ) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_CidrRoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_routing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `geolocation` after provisioning.\n"]
    pub fn geolocation(&self) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_GeolocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geolocation", self.base))
    }

    #[doc = "Get a reference to the value of field `geoproximity_location` after provisioning.\n"]
    pub fn geoproximity_location(
        &self,
    ) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_GeoproximityLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geoproximity_location", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\n"]
    pub fn resource_records(&self) -> terrars::ListRef<Route53RecordsExclusive_ResourceRecordSet_ResourceRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_records", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53RecordsExclusive_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53RecordsExclusive_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53RecordsExclusive_Timeouts {
    type O = terrars::BlockAssignable<Route53RecordsExclusive_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53RecordsExclusive_Timeouts {}

impl BuildRoute53RecordsExclusive_Timeouts {
    pub fn build(self) -> Route53RecordsExclusive_Timeouts {
        Route53RecordsExclusive_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53RecordsExclusive_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53RecordsExclusive_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Route53RecordsExclusive_TimeoutsRef {
        Route53RecordsExclusive_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53RecordsExclusive_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
