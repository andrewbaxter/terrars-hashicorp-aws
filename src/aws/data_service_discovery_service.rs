#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServiceDiscoveryServiceData_ {
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
    name: terrars::PrimField<std::string::String>,
    namespace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataServiceDiscoveryService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServiceDiscoveryServiceData_>,
}

#[derive(Clone)]
pub struct DataServiceDiscoveryService(std::rc::Rc<DataServiceDiscoveryService_>);

impl DataServiceDiscoveryService {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_HealthCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_HealthCheckCustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataServiceDiscoveryService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServiceDiscoveryService { }

impl terrars::ToListMappable for DataServiceDiscoveryService {
    type O = terrars::ListRef<DataServiceDiscoveryServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServiceDiscoveryService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_service_discovery_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServiceDiscoveryService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildDataServiceDiscoveryService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServiceDiscoveryService {
        let out = DataServiceDiscoveryService(std::rc::Rc::new(DataServiceDiscoveryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServiceDiscoveryServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                namespace_id: self.namespace_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServiceDiscoveryServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServiceDiscoveryServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_HealthCheckConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> terrars::ListRef<DataServiceDiscoveryService_HealthCheckCustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryService_DnsConfig_DnsRecords {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryService_DnsConfig_DnsRecords {
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
}

impl terrars::ToListMappable for DataServiceDiscoveryService_DnsConfig_DnsRecords {
    type O = terrars::BlockAssignable<DataServiceDiscoveryService_DnsConfig_DnsRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryService_DnsConfig_DnsRecords {}

impl BuildDataServiceDiscoveryService_DnsConfig_DnsRecords {
    pub fn build(self) -> DataServiceDiscoveryService_DnsConfig_DnsRecords {
        DataServiceDiscoveryService_DnsConfig_DnsRecords {
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataServiceDiscoveryService_DnsConfig_DnsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryService_DnsConfig_DnsRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryService_DnsConfig_DnsRecordsRef {
        DataServiceDiscoveryService_DnsConfig_DnsRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryService_DnsConfig_DnsRecordsRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataServiceDiscoveryService_DnsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_records: std::option::Option<terrars::ListField<DataServiceDiscoveryService_DnsConfig_DnsRecords>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryService_DnsConfig {
    #[doc = "Set the field `dns_records`.\n"]
    pub fn set_dns_records(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataServiceDiscoveryService_DnsConfig_DnsRecords>>,
    ) -> Self {
        self.dns_records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryService_DnsConfig {
    type O = terrars::BlockAssignable<DataServiceDiscoveryService_DnsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryService_DnsConfig {}

impl BuildDataServiceDiscoveryService_DnsConfig {
    pub fn build(self) -> DataServiceDiscoveryService_DnsConfig {
        DataServiceDiscoveryService_DnsConfig {
            dns_records: core::default::Default::default(),
            namespace_id: core::default::Default::default(),
            routing_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataServiceDiscoveryService_DnsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryService_DnsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataServiceDiscoveryService_DnsConfigRef {
        DataServiceDiscoveryService_DnsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryService_DnsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> terrars::ListRef<DataServiceDiscoveryService_DnsConfig_DnsRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryService_HealthCheckConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServiceDiscoveryService_HealthCheckConfig {
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

impl terrars::ToListMappable for DataServiceDiscoveryService_HealthCheckConfig {
    type O = terrars::BlockAssignable<DataServiceDiscoveryService_HealthCheckConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryService_HealthCheckConfig {}

impl BuildDataServiceDiscoveryService_HealthCheckConfig {
    pub fn build(self) -> DataServiceDiscoveryService_HealthCheckConfig {
        DataServiceDiscoveryService_HealthCheckConfig {
            failure_threshold: core::default::Default::default(),
            resource_path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataServiceDiscoveryService_HealthCheckConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryService_HealthCheckConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryService_HealthCheckConfigRef {
        DataServiceDiscoveryService_HealthCheckConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryService_HealthCheckConfigRef {
    #[allow(dead_code)]
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
pub struct DataServiceDiscoveryService_HealthCheckCustomConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataServiceDiscoveryService_HealthCheckCustomConfig {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServiceDiscoveryService_HealthCheckCustomConfig {
    type O = terrars::BlockAssignable<DataServiceDiscoveryService_HealthCheckCustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryService_HealthCheckCustomConfig {}

impl BuildDataServiceDiscoveryService_HealthCheckCustomConfig {
    pub fn build(self) -> DataServiceDiscoveryService_HealthCheckCustomConfig {
        DataServiceDiscoveryService_HealthCheckCustomConfig { failure_threshold: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataServiceDiscoveryService_HealthCheckCustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServiceDiscoveryService_HealthCheckCustomConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServiceDiscoveryService_HealthCheckCustomConfigRef {
        DataServiceDiscoveryService_HealthCheckCustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryService_HealthCheckCustomConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }
}
