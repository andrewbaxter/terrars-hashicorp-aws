#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53RecordsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct DataRoute53Records_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRoute53RecordsData_>,
}

#[derive(Clone)]
pub struct DataRoute53Records(std::rc::Rc<DataRoute53Records_>);

impl DataRoute53Records {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_record_sets` after provisioning.\n"]
    pub fn resource_record_sets(&self) -> terrars::ListRef<DataRoute53Records_ResourceRecordSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_record_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataRoute53Records {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRoute53Records { }

impl terrars::ToListMappable for DataRoute53Records {
    type O = terrars::ListRef<DataRoute53RecordsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRoute53Records_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_route53_records".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53Records {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute53Records {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRoute53Records {
        let out = DataRoute53Records(std::rc::Rc::new(DataRoute53Records_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRoute53RecordsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name_regex: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53RecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53RecordsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53RecordsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_record_sets` after provisioning.\n"]
    pub fn resource_record_sets(&self) -> terrars::ListRef<DataRoute53Records_ResourceRecordSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_record_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRoute53Records_ResourceRecordSets_AliasTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_AliasTarget {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_AliasTarget {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_AliasTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_AliasTarget {}

impl BuildDataRoute53Records_ResourceRecordSets_AliasTarget {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_AliasTarget {
        DataRoute53Records_ResourceRecordSets_AliasTarget {
            dns_name: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_AliasTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_AliasTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_AliasTargetRef {
        DataRoute53Records_ResourceRecordSets_AliasTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_AliasTargetRef {
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
pub struct DataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
    #[doc = "Set the field `collection_id`.\n"]
    pub fn set_collection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_name`.\n"]
    pub fn set_location_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_CidrRoutingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_CidrRoutingConfig {}

impl BuildDataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
        DataRoute53Records_ResourceRecordSets_CidrRoutingConfig {
            collection_id: core::default::Default::default(),
            location_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
        DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
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
pub struct DataRoute53Records_ResourceRecordSets_Geolocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continent_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdivision_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_Geolocation {
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

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_Geolocation {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_Geolocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_Geolocation {}

impl BuildDataRoute53Records_ResourceRecordSets_Geolocation {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_Geolocation {
        DataRoute53Records_ResourceRecordSets_Geolocation {
            continent_code: core::default::Default::default(),
            country_code: core::default::Default::default(),
            subdivision_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_GeolocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_GeolocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_GeolocationRef {
        DataRoute53Records_ResourceRecordSets_GeolocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_GeolocationRef {
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
pub struct DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
    #[doc = "Set the field `latitude`.\n"]
    pub fn set_latitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.latitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `longitude`.\n"]
    pub fn set_longitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.longitude = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {}

impl BuildDataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates {
            latitude: core::default::Default::default(),
            longitude: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
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

#[derive(Serialize)]
pub struct DataRoute53Records_ResourceRecordSets_GeoproximityLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bias: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coordinates: std::option::Option<DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_zone_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_GeoproximityLocation {
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

    #[doc = "Set the field `coordinates`.\n"]
    pub fn set_coordinates(
        mut self,
        v: impl std::convert::Into<DataRoute53Records_ResourceRecordSets_GeoproximityLocation_Coordinates>,
    ) -> Self {
        self.coordinates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_zone_group`.\n"]
    pub fn set_local_zone_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_zone_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_GeoproximityLocation {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_GeoproximityLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_GeoproximityLocation {}

impl BuildDataRoute53Records_ResourceRecordSets_GeoproximityLocation {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocation {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocation {
            aws_region: core::default::Default::default(),
            bias: core::default::Default::default(),
            coordinates: core::default::Default::default(),
            local_zone_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
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

    #[doc = "Get a reference to the value of field `coordinates` after provisioning.\n"]
    pub fn coordinates(&self) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocation_CoordinatesRef::new(
            self.shared().clone(),
            format!("{}.coordinates", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_zone_group` after provisioning.\n"]
    pub fn local_zone_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_zone_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53Records_ResourceRecordSets_ResourceRecords {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53Records_ResourceRecordSets_ResourceRecords {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets_ResourceRecords {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets_ResourceRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets_ResourceRecords {}

impl BuildDataRoute53Records_ResourceRecordSets_ResourceRecords {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets_ResourceRecords {
        DataRoute53Records_ResourceRecordSets_ResourceRecords { value: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSets_ResourceRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSets_ResourceRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53Records_ResourceRecordSets_ResourceRecordsRef {
        DataRoute53Records_ResourceRecordSets_ResourceRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSets_ResourceRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53Records_ResourceRecordSets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_target: std::option::Option<DataRoute53Records_ResourceRecordSets_AliasTarget>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_routing_config: std::option::Option<DataRoute53Records_ResourceRecordSets_CidrRoutingConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geolocation: std::option::Option<DataRoute53Records_ResourceRecordSets_Geolocation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geoproximity_location: std::option::Option<DataRoute53Records_ResourceRecordSets_GeoproximityLocation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_value_answer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_records: std::option::Option<terrars::ListField<DataRoute53Records_ResourceRecordSets_ResourceRecords>>,
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
}

impl DataRoute53Records_ResourceRecordSets {
    #[doc = "Set the field `alias_target`.\n"]
    pub fn set_alias_target(
        mut self,
        v: impl std::convert::Into<DataRoute53Records_ResourceRecordSets_AliasTarget>,
    ) -> Self {
        self.alias_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_routing_config`.\n"]
    pub fn set_cidr_routing_config(
        mut self,
        v: impl std::convert::Into<DataRoute53Records_ResourceRecordSets_CidrRoutingConfig>,
    ) -> Self {
        self.cidr_routing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failover`.\n"]
    pub fn set_failover(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failover = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geolocation`.\n"]
    pub fn set_geolocation(
        mut self,
        v: impl std::convert::Into<DataRoute53Records_ResourceRecordSets_Geolocation>,
    ) -> Self {
        self.geolocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geoproximity_location`.\n"]
    pub fn set_geoproximity_location(
        mut self,
        v: impl std::convert::Into<DataRoute53Records_ResourceRecordSets_GeoproximityLocation>,
    ) -> Self {
        self.geoproximity_location = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_records`.\n"]
    pub fn set_resource_records(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRoute53Records_ResourceRecordSets_ResourceRecords>>,
    ) -> Self {
        self.resource_records = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataRoute53Records_ResourceRecordSets {
    type O = terrars::BlockAssignable<DataRoute53Records_ResourceRecordSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53Records_ResourceRecordSets {}

impl BuildDataRoute53Records_ResourceRecordSets {
    pub fn build(self) -> DataRoute53Records_ResourceRecordSets {
        DataRoute53Records_ResourceRecordSets {
            alias_target: core::default::Default::default(),
            cidr_routing_config: core::default::Default::default(),
            failover: core::default::Default::default(),
            geolocation: core::default::Default::default(),
            geoproximity_location: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            multi_value_answer: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            resource_records: core::default::Default::default(),
            set_identifier: core::default::Default::default(),
            traffic_policy_instance_id: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRoute53Records_ResourceRecordSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53Records_ResourceRecordSetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRoute53Records_ResourceRecordSetsRef {
        DataRoute53Records_ResourceRecordSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53Records_ResourceRecordSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias_target` after provisioning.\n"]
    pub fn alias_target(&self) -> DataRoute53Records_ResourceRecordSets_AliasTargetRef {
        DataRoute53Records_ResourceRecordSets_AliasTargetRef::new(
            self.shared().clone(),
            format!("{}.alias_target", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_routing_config` after provisioning.\n"]
    pub fn cidr_routing_config(&self) -> DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef {
        DataRoute53Records_ResourceRecordSets_CidrRoutingConfigRef::new(
            self.shared().clone(),
            format!("{}.cidr_routing_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `failover` after provisioning.\n"]
    pub fn failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover", self.base))
    }

    #[doc = "Get a reference to the value of field `geolocation` after provisioning.\n"]
    pub fn geolocation(&self) -> DataRoute53Records_ResourceRecordSets_GeolocationRef {
        DataRoute53Records_ResourceRecordSets_GeolocationRef::new(
            self.shared().clone(),
            format!("{}.geolocation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `geoproximity_location` after provisioning.\n"]
    pub fn geoproximity_location(&self) -> DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef {
        DataRoute53Records_ResourceRecordSets_GeoproximityLocationRef::new(
            self.shared().clone(),
            format!("{}.geoproximity_location", self.base),
        )
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

    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\n"]
    pub fn resource_records(&self) -> terrars::ListRef<DataRoute53Records_ResourceRecordSets_ResourceRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_records", self.base))
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
}
