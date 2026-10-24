use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRoute53TrafficPolicyDocumentData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentEndpointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleEl>>,
    dynamic: DataRoute53TrafficPolicyDocumentDynamic,
}

struct DataRoute53TrafficPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRoute53TrafficPolicyDocumentData>,
}

#[derive(Clone)]
pub struct DataRoute53TrafficPolicyDocument(std::rc::Rc<DataRoute53TrafficPolicyDocument_>);

impl DataRoute53TrafficPolicyDocument {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_type`.\n"]
    pub fn set_record_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().record_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_endpoint`.\n"]
    pub fn set_start_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_rule`.\n"]
    pub fn set_start_rule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentEndpointEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_type` after provisioning.\n"]
    pub fn record_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_endpoint` after provisioning.\n"]
    pub fn start_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_rule` after provisioning.\n"]
    pub fn start_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataRoute53TrafficPolicyDocument {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRoute53TrafficPolicyDocument { }

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocument {
    type O = terrars::ListRef<DataRoute53TrafficPolicyDocumentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRoute53TrafficPolicyDocument_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_route53_traffic_policy_document".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute53TrafficPolicyDocument {
    pub tf_id: std::string::String,
}

impl BuildDataRoute53TrafficPolicyDocument {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRoute53TrafficPolicyDocument {
        let out = DataRoute53TrafficPolicyDocument(std::rc::Rc::new(DataRoute53TrafficPolicyDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRoute53TrafficPolicyDocumentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                record_type: core::default::Default::default(),
                start_endpoint: core::default::Default::default(),
                start_rule: core::default::Default::default(),
                version: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRoute53TrafficPolicyDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_type` after provisioning.\n"]
    pub fn record_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_endpoint` after provisioning.\n"]
    pub fn start_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_rule` after provisioning.\n"]
    pub fn start_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentEndpointEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentEndpointEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentEndpointEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentEndpointEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute53TrafficPolicyDocumentEndpointEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentEndpointEl {
        DataRoute53TrafficPolicyDocumentEndpointEl {
            id: self.id,
            region: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentEndpointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRoute53TrafficPolicyDocumentEndpointElRef {
        DataRoute53TrafficPolicyDocumentEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_reference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
    #[doc = "Set the field `bias`.\n"]
    pub fn set_bias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_reference`.\n"]
    pub fn set_rule_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
        DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl {
            bias: core::default::Default::default(),
            endpoint_reference: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            health_check: core::default::Default::default(),
            latitude: core::default::Default::default(),
            longitude: core::default::Default::default(),
            region: core::default::Default::default(),
            rule_reference: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationElRef {
        DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bias` after provisioning.\n"]
    pub fn bias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bias", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `latitude` after provisioning.\n"]
    pub fn latitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc = "Get a reference to the value of field `longitude` after provisioning.\n"]
    pub fn longitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_reference` after provisioning.\n"]
    pub fn rule_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElItemsEl {
    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElItemsEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElItemsEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElItemsEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElItemsEl {
        DataRoute53TrafficPolicyDocumentRuleElItemsEl {
            endpoint_reference: core::default::Default::default(),
            health_check: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElItemsElRef {
        DataRoute53TrafficPolicyDocumentRuleElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElItemsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdivision: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElLocationEl {
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

    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_default`.\n"]
    pub fn set_is_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_reference`.\n"]
    pub fn set_rule_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdivision`.\n"]
    pub fn set_subdivision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subdivision = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElLocationEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElLocationEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElLocationEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElLocationEl {
        DataRoute53TrafficPolicyDocumentRuleElLocationEl {
            continent: core::default::Default::default(),
            country: core::default::Default::default(),
            endpoint_reference: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            health_check: core::default::Default::default(),
            is_default: core::default::Default::default(),
            rule_reference: core::default::Default::default(),
            subdivision: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElLocationElRef {
        DataRoute53TrafficPolicyDocumentRuleElLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElLocationElRef {
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

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_reference` after provisioning.\n"]
    pub fn rule_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `subdivision` after provisioning.\n"]
    pub fn subdivision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdivision", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_reference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_reference`.\n"]
    pub fn set_rule_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElPrimaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElPrimaryEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
        DataRoute53TrafficPolicyDocumentRuleElPrimaryEl {
            endpoint_reference: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            health_check: core::default::Default::default(),
            rule_reference: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef {
        DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_reference` after provisioning.\n"]
    pub fn rule_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElRegionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_reference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElRegionEl {
    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_reference`.\n"]
    pub fn set_rule_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElRegionEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElRegionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElRegionEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElRegionEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElRegionEl {
        DataRoute53TrafficPolicyDocumentRuleElRegionEl {
            endpoint_reference: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            health_check: core::default::Default::default(),
            region: core::default::Default::default(),
            rule_reference: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElRegionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElRegionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElRegionElRef {
        DataRoute53TrafficPolicyDocumentRuleElRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElRegionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_reference` after provisioning.\n"]
    pub fn rule_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_target_health: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_reference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
    #[doc = "Set the field `endpoint_reference`.\n"]
    pub fn set_endpoint_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_target_health`.\n"]
    pub fn set_evaluate_target_health(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.evaluate_target_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_reference`.\n"]
    pub fn set_rule_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElSecondaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleElSecondaryEl {}

impl BuildDataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
        DataRoute53TrafficPolicyDocumentRuleElSecondaryEl {
            endpoint_reference: core::default::Default::default(),
            evaluate_target_health: core::default::Default::default(),
            health_check: core::default::Default::default(),
            rule_reference: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef {
        DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_reference` after provisioning.\n"]
    pub fn endpoint_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_target_health` after provisioning.\n"]
    pub fn evaluate_target_health(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluate_target_health", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_reference` after provisioning.\n"]
    pub fn rule_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_reference", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRoute53TrafficPolicyDocumentRuleElDynamic {
    geo_proximity_location: std
    ::option
    ::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl>>,
    items: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElItemsEl>>,
    location: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElLocationEl>>,
    primary: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElPrimaryEl>>,
    region: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElRegionEl>>,
    secondary: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleElSecondaryEl>>,
}

#[derive(Serialize)]
pub struct DataRoute53TrafficPolicyDocumentRuleEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_proximity_location: std
    ::option
    ::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElItemsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElPrimaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElRegionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary: std::option::Option<std::vec::Vec<DataRoute53TrafficPolicyDocumentRuleElSecondaryEl>>,
    dynamic: DataRoute53TrafficPolicyDocumentRuleElDynamic,
}

impl DataRoute53TrafficPolicyDocumentRuleEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo_proximity_location`.\n"]
    pub fn set_geo_proximity_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElGeoProximityLocationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_proximity_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_proximity_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElItemsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElLocationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElPrimaryEl>>,
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

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElRegionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary`.\n"]
    pub fn set_secondary(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleElSecondaryEl>>,
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

impl terrars::ToListMappable for DataRoute53TrafficPolicyDocumentRuleEl {
    type O = terrars::BlockAssignable<DataRoute53TrafficPolicyDocumentRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRoute53TrafficPolicyDocumentRuleEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute53TrafficPolicyDocumentRuleEl {
    pub fn build(self) -> DataRoute53TrafficPolicyDocumentRuleEl {
        DataRoute53TrafficPolicyDocumentRuleEl {
            id: self.id,
            type_: core::default::Default::default(),
            geo_proximity_location: core::default::Default::default(),
            items: core::default::Default::default(),
            location: core::default::Default::default(),
            primary: core::default::Default::default(),
            region: core::default::Default::default(),
            secondary: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataRoute53TrafficPolicyDocumentRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRoute53TrafficPolicyDocumentRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRoute53TrafficPolicyDocumentRuleElRef {
        DataRoute53TrafficPolicyDocumentRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRoute53TrafficPolicyDocumentRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> terrars::ListRef<DataRoute53TrafficPolicyDocumentRuleElPrimaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary` after provisioning.\n"]
    pub fn secondary(&self) -> terrars::ListRef<DataRoute53TrafficPolicyDocumentRuleElSecondaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRoute53TrafficPolicyDocumentDynamic {
    endpoint: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentEndpointEl>>,
    rule: std::option::Option<terrars::DynamicBlock<DataRoute53TrafficPolicyDocumentRuleEl>>,
}
