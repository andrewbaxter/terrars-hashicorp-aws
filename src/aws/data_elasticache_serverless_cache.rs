#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticacheServerlessCacheData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataElasticacheServerlessCache_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataElasticacheServerlessCacheData_>,
}

#[derive(Clone)]
pub struct DataElasticacheServerlessCache(std::rc::Rc<DataElasticacheServerlessCache_>);

impl DataElasticacheServerlessCache {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> DataElasticacheServerlessCache_CacheUsageLimitsRef {
        DataElasticacheServerlessCache_CacheUsageLimitsRef::new(
            self.shared().clone(),
            format!("{}.cache_usage_limits", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daily_snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> DataElasticacheServerlessCache_EndpointRef {
        DataElasticacheServerlessCache_EndpointRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> DataElasticacheServerlessCache_ReaderEndpointRef {
        DataElasticacheServerlessCache_ReaderEndpointRef::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataElasticacheServerlessCache {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataElasticacheServerlessCache { }

impl terrars::ToListMappable for DataElasticacheServerlessCache {
    type O = terrars::ListRef<DataElasticacheServerlessCacheRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataElasticacheServerlessCache_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_elasticache_serverless_cache".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticacheServerlessCache {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataElasticacheServerlessCache {
    pub fn build(self, stack: &mut terrars::Stack) -> DataElasticacheServerlessCache {
        let out = DataElasticacheServerlessCache(std::rc::Rc::new(DataElasticacheServerlessCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataElasticacheServerlessCacheData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticacheServerlessCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElasticacheServerlessCacheRef {
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

    #[doc = "Get a reference to the value of field `cache_usage_limits` after provisioning.\n"]
    pub fn cache_usage_limits(&self) -> DataElasticacheServerlessCache_CacheUsageLimitsRef {
        DataElasticacheServerlessCache_CacheUsageLimitsRef::new(
            self.shared().clone(),
            format!("{}.cache_usage_limits", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_snapshot_time` after provisioning.\n"]
    pub fn daily_snapshot_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daily_snapshot_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> DataElasticacheServerlessCache_EndpointRef {
        DataElasticacheServerlessCache_EndpointRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_engine_version` after provisioning.\n"]
    pub fn full_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `major_engine_version` after provisioning.\n"]
    pub fn major_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_endpoint` after provisioning.\n"]
    pub fn reader_endpoint(&self) -> DataElasticacheServerlessCache_ReaderEndpointRef {
        DataElasticacheServerlessCache_ReaderEndpointRef::new(
            self.shared().clone(),
            format!("{}.reader_endpoint", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_limit` after provisioning.\n"]
    pub fn snapshot_retention_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\n"]
    pub fn user_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
    type O = terrars::BlockAssignable<DataElasticacheServerlessCache_CacheUsageLimits_DataStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServerlessCache_CacheUsageLimits_DataStorage {}

impl BuildDataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
    pub fn build(self) -> DataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
        DataElasticacheServerlessCache_CacheUsageLimits_DataStorage {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
        DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum: std::option::Option<terrars::PrimField<f64>>,
}

impl DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
    type O = terrars::BlockAssignable<DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {}

impl BuildDataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
    pub fn build(self) -> DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
        DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
        DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServerlessCache_CacheUsageLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage: std::option::Option<DataElasticacheServerlessCache_CacheUsageLimits_DataStorage>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecpu_per_second: std::option::Option<DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond>,
}

impl DataElasticacheServerlessCache_CacheUsageLimits {
    #[doc = "Set the field `data_storage`.\n"]
    pub fn set_data_storage(
        mut self,
        v: impl std::convert::Into<DataElasticacheServerlessCache_CacheUsageLimits_DataStorage>,
    ) -> Self {
        self.data_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecpu_per_second`.\n"]
    pub fn set_ecpu_per_second(
        mut self,
        v: impl std::convert::Into<DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecond>,
    ) -> Self {
        self.ecpu_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServerlessCache_CacheUsageLimits {
    type O = terrars::BlockAssignable<DataElasticacheServerlessCache_CacheUsageLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServerlessCache_CacheUsageLimits {}

impl BuildDataElasticacheServerlessCache_CacheUsageLimits {
    pub fn build(self) -> DataElasticacheServerlessCache_CacheUsageLimits {
        DataElasticacheServerlessCache_CacheUsageLimits {
            data_storage: core::default::Default::default(),
            ecpu_per_second: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServerlessCache_CacheUsageLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCache_CacheUsageLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheServerlessCache_CacheUsageLimitsRef {
        DataElasticacheServerlessCache_CacheUsageLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServerlessCache_CacheUsageLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_storage` after provisioning.\n"]
    pub fn data_storage(&self) -> DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef {
        DataElasticacheServerlessCache_CacheUsageLimits_DataStorageRef::new(
            self.shared().clone(),
            format!("{}.data_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ecpu_per_second` after provisioning.\n"]
    pub fn ecpu_per_second(&self) -> DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef {
        DataElasticacheServerlessCache_CacheUsageLimits_EcpuPerSecondRef::new(
            self.shared().clone(),
            format!("{}.ecpu_per_second", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServerlessCache_Endpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataElasticacheServerlessCache_Endpoint {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServerlessCache_Endpoint {
    type O = terrars::BlockAssignable<DataElasticacheServerlessCache_Endpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServerlessCache_Endpoint {}

impl BuildDataElasticacheServerlessCache_Endpoint {
    pub fn build(self) -> DataElasticacheServerlessCache_Endpoint {
        DataElasticacheServerlessCache_Endpoint {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServerlessCache_EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCache_EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataElasticacheServerlessCache_EndpointRef {
        DataElasticacheServerlessCache_EndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServerlessCache_EndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElasticacheServerlessCache_ReaderEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataElasticacheServerlessCache_ReaderEndpoint {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticacheServerlessCache_ReaderEndpoint {
    type O = terrars::BlockAssignable<DataElasticacheServerlessCache_ReaderEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticacheServerlessCache_ReaderEndpoint {}

impl BuildDataElasticacheServerlessCache_ReaderEndpoint {
    pub fn build(self) -> DataElasticacheServerlessCache_ReaderEndpoint {
        DataElasticacheServerlessCache_ReaderEndpoint {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataElasticacheServerlessCache_ReaderEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticacheServerlessCache_ReaderEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticacheServerlessCache_ReaderEndpointRef {
        DataElasticacheServerlessCache_ReaderEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticacheServerlessCache_ReaderEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
