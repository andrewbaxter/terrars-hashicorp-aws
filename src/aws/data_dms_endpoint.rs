#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDmsEndpointData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    endpoint_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataDmsEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDmsEndpointData_>,
}

#[derive(Clone)]
pub struct DataDmsEndpoint(std::rc::Rc<DataDmsEndpoint_>);

impl DataDmsEndpoint {
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

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> terrars::ListRef<DataDmsEndpoint_ElasticsearchSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extra_connection_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> terrars::ListRef<DataDmsEndpoint_KafkaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> terrars::ListRef<DataDmsEndpoint_KinesisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> terrars::ListRef<DataDmsEndpoint_MongodbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> terrars::ListRef<DataDmsEndpoint_MysqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> terrars::ListRef<DataDmsEndpoint_PostgresSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> terrars::ListRef<DataDmsEndpoint_RedisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> terrars::ListRef<DataDmsEndpoint_RedshiftSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_settings` after provisioning.\n"]
    pub fn s3_settings(&self) -> terrars::ListRef<DataDmsEndpoint_S3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_access_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl terrars::Referable for DataDmsEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDmsEndpoint { }

impl terrars::ToListMappable for DataDmsEndpoint {
    type O = terrars::ListRef<DataDmsEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDmsEndpoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_dms_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDmsEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub endpoint_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDmsEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDmsEndpoint {
        let out = DataDmsEndpoint(std::rc::Rc::new(DataDmsEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDmsEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                endpoint_id: self.endpoint_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDmsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDmsEndpointRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> terrars::ListRef<DataDmsEndpoint_ElasticsearchSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extra_connection_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> terrars::ListRef<DataDmsEndpoint_KafkaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> terrars::ListRef<DataDmsEndpoint_KinesisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> terrars::ListRef<DataDmsEndpoint_MongodbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> terrars::ListRef<DataDmsEndpoint_MysqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> terrars::ListRef<DataDmsEndpoint_PostgresSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> terrars::ListRef<DataDmsEndpoint_RedisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> terrars::ListRef<DataDmsEndpoint_RedshiftSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_settings` after provisioning.\n"]
    pub fn s3_settings(&self) -> terrars::ListRef<DataDmsEndpoint_S3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_access_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_ElasticsearchSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_retry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_load_error_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_ElasticsearchSettings {
    #[doc = "Set the field `endpoint_uri`.\n"]
    pub fn set_endpoint_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_retry_duration`.\n"]
    pub fn set_error_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.error_retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `full_load_error_percentage`.\n"]
    pub fn set_full_load_error_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.full_load_error_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_ElasticsearchSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_ElasticsearchSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_ElasticsearchSettings {}

impl BuildDataDmsEndpoint_ElasticsearchSettings {
    pub fn build(self) -> DataDmsEndpoint_ElasticsearchSettings {
        DataDmsEndpoint_ElasticsearchSettings {
            endpoint_uri: core::default::Default::default(),
            error_retry_duration: core::default::Default::default(),
            full_load_error_percentage: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_ElasticsearchSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_ElasticsearchSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_ElasticsearchSettingsRef {
        DataDmsEndpoint_ElasticsearchSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_ElasticsearchSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\n"]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `error_retry_duration` after provisioning.\n"]
    pub fn error_retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `full_load_error_percentage` after provisioning.\n"]
    pub fn full_load_error_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_load_error_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_KafkaSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_control_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_null_and_empty: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_partition_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_table_alter_operations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_transaction_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_max_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_hex_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_include_schema_table: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_mechanism: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_client_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_client_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_client_key_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_KafkaSettings {
    #[doc = "Set the field `broker`.\n"]
    pub fn set_broker(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.broker = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_control_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_null_and_empty = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_partition_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_table_alter_operations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_transaction_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_max_bytes`.\n"]
    pub fn set_message_max_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.message_max_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_hex_prefix`.\n"]
    pub fn set_no_hex_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_hex_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.partition_include_schema_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_mechanism`.\n"]
    pub fn set_sasl_mechanism(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sasl_mechanism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_password`.\n"]
    pub fn set_sasl_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sasl_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_username`.\n"]
    pub fn set_sasl_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sasl_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\n"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_client_certificate_arn`.\n"]
    pub fn set_ssl_client_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_client_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_client_key_arn`.\n"]
    pub fn set_ssl_client_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_client_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_client_key_password`.\n"]
    pub fn set_ssl_client_key_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_client_key_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_KafkaSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_KafkaSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_KafkaSettings {}

impl BuildDataDmsEndpoint_KafkaSettings {
    pub fn build(self) -> DataDmsEndpoint_KafkaSettings {
        DataDmsEndpoint_KafkaSettings {
            broker: core::default::Default::default(),
            include_control_details: core::default::Default::default(),
            include_null_and_empty: core::default::Default::default(),
            include_partition_value: core::default::Default::default(),
            include_table_alter_operations: core::default::Default::default(),
            include_transaction_details: core::default::Default::default(),
            message_format: core::default::Default::default(),
            message_max_bytes: core::default::Default::default(),
            no_hex_prefix: core::default::Default::default(),
            partition_include_schema_table: core::default::Default::default(),
            sasl_mechanism: core::default::Default::default(),
            sasl_password: core::default::Default::default(),
            sasl_username: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_ca_certificate_arn: core::default::Default::default(),
            ssl_client_certificate_arn: core::default::Default::default(),
            ssl_client_key_arn: core::default::Default::default(),
            ssl_client_key_password: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_KafkaSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_KafkaSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_KafkaSettingsRef {
        DataDmsEndpoint_KafkaSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_KafkaSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `broker` after provisioning.\n"]
    pub fn broker(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker", self.base))
    }

    #[doc = "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_control_details", self.base))
    }

    #[doc = "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_null_and_empty", self.base))
    }

    #[doc = "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_partition_value", self.base))
    }

    #[doc = "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_table_alter_operations", self.base))
    }

    #[doc = "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_transaction_details", self.base))
    }

    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc = "Get a reference to the value of field `message_max_bytes` after provisioning.\n"]
    pub fn message_max_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_max_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `no_hex_prefix` after provisioning.\n"]
    pub fn no_hex_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_hex_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_include_schema_table", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_mechanism` after provisioning.\n"]
    pub fn sasl_mechanism(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_mechanism", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_password` after provisioning.\n"]
    pub fn sasl_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_password", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_username` after provisioning.\n"]
    pub fn sasl_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_username", self.base))
    }

    #[doc = "Get a reference to the value of field `security_protocol` after provisioning.\n"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_client_certificate_arn` after provisioning.\n"]
    pub fn ssl_client_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_client_key_arn` after provisioning.\n"]
    pub fn ssl_client_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_client_key_password` after provisioning.\n"]
    pub fn ssl_client_key_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_key_password", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_KinesisSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_control_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_null_and_empty: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_partition_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_table_alter_operations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_transaction_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_include_schema_table: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_large_integer_value: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDmsEndpoint_KinesisSettings {
    #[doc = "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_control_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_null_and_empty = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_partition_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_table_alter_operations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_transaction_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.partition_include_schema_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_large_integer_value`.\n"]
    pub fn set_use_large_integer_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_large_integer_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_KinesisSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_KinesisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_KinesisSettings {}

impl BuildDataDmsEndpoint_KinesisSettings {
    pub fn build(self) -> DataDmsEndpoint_KinesisSettings {
        DataDmsEndpoint_KinesisSettings {
            include_control_details: core::default::Default::default(),
            include_null_and_empty: core::default::Default::default(),
            include_partition_value: core::default::Default::default(),
            include_table_alter_operations: core::default::Default::default(),
            include_transaction_details: core::default::Default::default(),
            message_format: core::default::Default::default(),
            partition_include_schema_table: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
            use_large_integer_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_KinesisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_KinesisSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_KinesisSettingsRef {
        DataDmsEndpoint_KinesisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_KinesisSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_control_details", self.base))
    }

    #[doc = "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_null_and_empty", self.base))
    }

    #[doc = "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_partition_value", self.base))
    }

    #[doc = "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_table_alter_operations", self.base))
    }

    #[doc = "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_transaction_details", self.base))
    }

    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_include_schema_table", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `use_large_integer_value` after provisioning.\n"]
    pub fn use_large_integer_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_large_integer_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_MongodbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_mechanism: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docs_to_investigate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extract_doc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nesting_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_update_lookup: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDmsEndpoint_MongodbSettings {
    #[doc = "Set the field `auth_mechanism`.\n"]
    pub fn set_auth_mechanism(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_mechanism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_source`.\n"]
    pub fn set_auth_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_type`.\n"]
    pub fn set_auth_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docs_to_investigate`.\n"]
    pub fn set_docs_to_investigate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.docs_to_investigate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extract_doc_id`.\n"]
    pub fn set_extract_doc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.extract_doc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nesting_level`.\n"]
    pub fn set_nesting_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nesting_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_update_lookup`.\n"]
    pub fn set_use_update_lookup(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_update_lookup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_MongodbSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_MongodbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_MongodbSettings {}

impl BuildDataDmsEndpoint_MongodbSettings {
    pub fn build(self) -> DataDmsEndpoint_MongodbSettings {
        DataDmsEndpoint_MongodbSettings {
            auth_mechanism: core::default::Default::default(),
            auth_source: core::default::Default::default(),
            auth_type: core::default::Default::default(),
            docs_to_investigate: core::default::Default::default(),
            extract_doc_id: core::default::Default::default(),
            nesting_level: core::default::Default::default(),
            use_update_lookup: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_MongodbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_MongodbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_MongodbSettingsRef {
        DataDmsEndpoint_MongodbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_MongodbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_mechanism` after provisioning.\n"]
    pub fn auth_mechanism(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mechanism", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_source` after provisioning.\n"]
    pub fn auth_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_source", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `docs_to_investigate` after provisioning.\n"]
    pub fn docs_to_investigate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docs_to_investigate", self.base))
    }

    #[doc = "Get a reference to the value of field `extract_doc_id` after provisioning.\n"]
    pub fn extract_doc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extract_doc_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nesting_level` after provisioning.\n"]
    pub fn nesting_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nesting_level", self.base))
    }

    #[doc = "Get a reference to the value of field `use_update_lookup` after provisioning.\n"]
    pub fn use_update_lookup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_update_lookup", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_MysqlSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_connect_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clean_source_metadata_on_mismatch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events_poll_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execute_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallel_load_threads: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_db_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_MysqlSettings {
    #[doc = "Set the field `after_connect_script`.\n"]
    pub fn set_after_connect_script(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.after_connect_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_method`.\n"]
    pub fn set_authentication_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clean_source_metadata_on_mismatch`.\n"]
    pub fn set_clean_source_metadata_on_mismatch(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.clean_source_metadata_on_mismatch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `events_poll_interval`.\n"]
    pub fn set_events_poll_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.events_poll_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execute_timeout`.\n"]
    pub fn set_execute_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.execute_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallel_load_threads`.\n"]
    pub fn set_parallel_load_threads(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallel_load_threads = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_timezone`.\n"]
    pub fn set_server_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_db_type`.\n"]
    pub fn set_target_db_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_db_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_MysqlSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_MysqlSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_MysqlSettings {}

impl BuildDataDmsEndpoint_MysqlSettings {
    pub fn build(self) -> DataDmsEndpoint_MysqlSettings {
        DataDmsEndpoint_MysqlSettings {
            after_connect_script: core::default::Default::default(),
            authentication_method: core::default::Default::default(),
            clean_source_metadata_on_mismatch: core::default::Default::default(),
            events_poll_interval: core::default::Default::default(),
            execute_timeout: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            parallel_load_threads: core::default::Default::default(),
            server_timezone: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            target_db_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_MysqlSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_MysqlSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_MysqlSettingsRef {
        DataDmsEndpoint_MysqlSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_MysqlSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_connect_script` after provisioning.\n"]
    pub fn after_connect_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after_connect_script", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_method", self.base))
    }

    #[doc = "Get a reference to the value of field `clean_source_metadata_on_mismatch` after provisioning.\n"]
    pub fn clean_source_metadata_on_mismatch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clean_source_metadata_on_mismatch", self.base))
    }

    #[doc = "Get a reference to the value of field `events_poll_interval` after provisioning.\n"]
    pub fn events_poll_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.events_poll_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `execute_timeout` after provisioning.\n"]
    pub fn execute_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execute_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc = "Get a reference to the value of field `parallel_load_threads` after provisioning.\n"]
    pub fn parallel_load_threads(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallel_load_threads", self.base))
    }

    #[doc = "Get a reference to the value of field `server_timezone` after provisioning.\n"]
    pub fn server_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_db_type` after provisioning.\n"]
    pub fn target_db_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_db_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_PostgresSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_connect_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    babelfish_database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capture_ddls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ddl_artifacts_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execute_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_tasks_on_lob_truncation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    heartbeat_enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    heartbeat_frequency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    heartbeat_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_boolean_as_boolean: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_jsonb_as_clob: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_long_varchar_as: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugin_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slot_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_PostgresSettings {
    #[doc = "Set the field `after_connect_script`.\n"]
    pub fn set_after_connect_script(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.after_connect_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_method`.\n"]
    pub fn set_authentication_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `babelfish_database_name`.\n"]
    pub fn set_babelfish_database_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.babelfish_database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capture_ddls`.\n"]
    pub fn set_capture_ddls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.capture_ddls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_mode`.\n"]
    pub fn set_database_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ddl_artifacts_schema`.\n"]
    pub fn set_ddl_artifacts_schema(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ddl_artifacts_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execute_timeout`.\n"]
    pub fn set_execute_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.execute_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_tasks_on_lob_truncation`.\n"]
    pub fn set_fail_tasks_on_lob_truncation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_tasks_on_lob_truncation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `heartbeat_enable`.\n"]
    pub fn set_heartbeat_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.heartbeat_enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `heartbeat_frequency`.\n"]
    pub fn set_heartbeat_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.heartbeat_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `heartbeat_schema`.\n"]
    pub fn set_heartbeat_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.heartbeat_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_boolean_as_boolean`.\n"]
    pub fn set_map_boolean_as_boolean(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.map_boolean_as_boolean = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_jsonb_as_clob`.\n"]
    pub fn set_map_jsonb_as_clob(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.map_jsonb_as_clob = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_long_varchar_as`.\n"]
    pub fn set_map_long_varchar_as(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.map_long_varchar_as = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plugin_name`.\n"]
    pub fn set_plugin_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plugin_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slot_name`.\n"]
    pub fn set_slot_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slot_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_PostgresSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_PostgresSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_PostgresSettings {}

impl BuildDataDmsEndpoint_PostgresSettings {
    pub fn build(self) -> DataDmsEndpoint_PostgresSettings {
        DataDmsEndpoint_PostgresSettings {
            after_connect_script: core::default::Default::default(),
            authentication_method: core::default::Default::default(),
            babelfish_database_name: core::default::Default::default(),
            capture_ddls: core::default::Default::default(),
            database_mode: core::default::Default::default(),
            ddl_artifacts_schema: core::default::Default::default(),
            execute_timeout: core::default::Default::default(),
            fail_tasks_on_lob_truncation: core::default::Default::default(),
            heartbeat_enable: core::default::Default::default(),
            heartbeat_frequency: core::default::Default::default(),
            heartbeat_schema: core::default::Default::default(),
            map_boolean_as_boolean: core::default::Default::default(),
            map_jsonb_as_clob: core::default::Default::default(),
            map_long_varchar_as: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            plugin_name: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            slot_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_PostgresSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_PostgresSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_PostgresSettingsRef {
        DataDmsEndpoint_PostgresSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_PostgresSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_connect_script` after provisioning.\n"]
    pub fn after_connect_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after_connect_script", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_method", self.base))
    }

    #[doc = "Get a reference to the value of field `babelfish_database_name` after provisioning.\n"]
    pub fn babelfish_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.babelfish_database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `capture_ddls` after provisioning.\n"]
    pub fn capture_ddls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capture_ddls", self.base))
    }

    #[doc = "Get a reference to the value of field `database_mode` after provisioning.\n"]
    pub fn database_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `ddl_artifacts_schema` after provisioning.\n"]
    pub fn ddl_artifacts_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ddl_artifacts_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `execute_timeout` after provisioning.\n"]
    pub fn execute_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execute_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_tasks_on_lob_truncation` after provisioning.\n"]
    pub fn fail_tasks_on_lob_truncation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_tasks_on_lob_truncation", self.base))
    }

    #[doc = "Get a reference to the value of field `heartbeat_enable` after provisioning.\n"]
    pub fn heartbeat_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.heartbeat_enable", self.base))
    }

    #[doc = "Get a reference to the value of field `heartbeat_frequency` after provisioning.\n"]
    pub fn heartbeat_frequency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.heartbeat_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `heartbeat_schema` after provisioning.\n"]
    pub fn heartbeat_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.heartbeat_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `map_boolean_as_boolean` after provisioning.\n"]
    pub fn map_boolean_as_boolean(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_boolean_as_boolean", self.base))
    }

    #[doc = "Get a reference to the value of field `map_jsonb_as_clob` after provisioning.\n"]
    pub fn map_jsonb_as_clob(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_jsonb_as_clob", self.base))
    }

    #[doc = "Get a reference to the value of field `map_long_varchar_as` after provisioning.\n"]
    pub fn map_long_varchar_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_long_varchar_as", self.base))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc = "Get a reference to the value of field `plugin_name` after provisioning.\n"]
    pub fn plugin_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `slot_name` after provisioning.\n"]
    pub fn slot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slot_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_RedisSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_RedisSettings {
    #[doc = "Set the field `auth_password`.\n"]
    pub fn set_auth_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_type`.\n"]
    pub fn set_auth_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_user_name`.\n"]
    pub fn set_auth_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_user_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_ca_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_security_protocol`.\n"]
    pub fn set_ssl_security_protocol(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_security_protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_RedisSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_RedisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_RedisSettings {}

impl BuildDataDmsEndpoint_RedisSettings {
    pub fn build(self) -> DataDmsEndpoint_RedisSettings {
        DataDmsEndpoint_RedisSettings {
            auth_password: core::default::Default::default(),
            auth_type: core::default::Default::default(),
            auth_user_name: core::default::Default::default(),
            port: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_ca_certificate_arn: core::default::Default::default(),
            ssl_security_protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_RedisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_RedisSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_RedisSettingsRef {
        DataDmsEndpoint_RedisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_RedisSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_password` after provisioning.\n"]
    pub fn auth_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_password", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_user_name` after provisioning.\n"]
    pub fn auth_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_user_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_security_protocol` after provisioning.\n"]
    pub fn ssl_security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_security_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_RedshiftSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_folder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDmsEndpoint_RedshiftSettings {
    #[doc = "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_side_encryption_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_RedshiftSettings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_RedshiftSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_RedshiftSettings {}

impl BuildDataDmsEndpoint_RedshiftSettings {
    pub fn build(self) -> DataDmsEndpoint_RedshiftSettings {
        DataDmsEndpoint_RedshiftSettings {
            bucket_folder: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            encryption_mode: core::default::Default::default(),
            server_side_encryption_kms_key_id: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_RedshiftSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_RedshiftSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_RedshiftSettingsRef {
        DataDmsEndpoint_RedshiftSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_RedshiftSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDmsEndpoint_S3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_column_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_folder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl_for_objects: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_inserts_and_updates: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_inserts_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_max_batch_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_min_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_no_sup_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_null_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_row_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_page_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_sequence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dict_page_size_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_statistics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_table_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_catalog_generation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_header_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_headers_row: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_op_for_full_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_timestamp_in_millisecond: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_transactions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rfc_4180: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_group_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_csv_no_sup_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_task_start_time_for_full_load_timestamp: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDmsEndpoint_S3Settings {
    #[doc = "Set the field `add_column_name`.\n"]
    pub fn set_add_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.add_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canned_acl_for_objects`.\n"]
    pub fn set_canned_acl_for_objects(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.canned_acl_for_objects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_and_updates`.\n"]
    pub fn set_cdc_inserts_and_updates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cdc_inserts_and_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_only`.\n"]
    pub fn set_cdc_inserts_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cdc_inserts_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_max_batch_interval`.\n"]
    pub fn set_cdc_max_batch_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cdc_max_batch_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_min_file_size`.\n"]
    pub fn set_cdc_min_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cdc_min_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_path`.\n"]
    pub fn set_cdc_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cdc_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_delimiter`.\n"]
    pub fn set_csv_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.csv_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_no_sup_value`.\n"]
    pub fn set_csv_no_sup_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.csv_no_sup_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_null_value`.\n"]
    pub fn set_csv_null_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.csv_null_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_row_delimiter`.\n"]
    pub fn set_csv_row_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.csv_row_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_page_size`.\n"]
    pub fn set_data_page_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_page_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_delimiter`.\n"]
    pub fn set_date_partition_delimiter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.date_partition_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_enabled`.\n"]
    pub fn set_date_partition_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.date_partition_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_sequence`.\n"]
    pub fn set_date_partition_sequence(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.date_partition_sequence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dict_page_size_limit`.\n"]
    pub fn set_dict_page_size_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dict_page_size_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_statistics`.\n"]
    pub fn set_enable_statistics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding_type`.\n"]
    pub fn set_encoding_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_table_definition`.\n"]
    pub fn set_external_table_definition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.external_table_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `glue_catalog_generation`.\n"]
    pub fn set_glue_catalog_generation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.glue_catalog_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_header_rows`.\n"]
    pub fn set_ignore_header_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ignore_header_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_headers_row`.\n"]
    pub fn set_ignore_headers_row(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ignore_headers_row = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_op_for_full_load`.\n"]
    pub fn set_include_op_for_full_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_op_for_full_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_timestamp_in_millisecond`.\n"]
    pub fn set_parquet_timestamp_in_millisecond(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.parquet_timestamp_in_millisecond = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_version`.\n"]
    pub fn set_parquet_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parquet_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_transactions`.\n"]
    pub fn set_preserve_transactions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_transactions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rfc_4180`.\n"]
    pub fn set_rfc_4180(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rfc_4180 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_group_length`.\n"]
    pub fn set_row_group_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_group_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_side_encryption_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_column_name`.\n"]
    pub fn set_timestamp_column_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timestamp_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_csv_no_sup_value`.\n"]
    pub fn set_use_csv_no_sup_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_csv_no_sup_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_task_start_time_for_full_load_timestamp`.\n"]
    pub fn set_use_task_start_time_for_full_load_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.use_task_start_time_for_full_load_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDmsEndpoint_S3Settings {
    type O = terrars::BlockAssignable<DataDmsEndpoint_S3Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDmsEndpoint_S3Settings {}

impl BuildDataDmsEndpoint_S3Settings {
    pub fn build(self) -> DataDmsEndpoint_S3Settings {
        DataDmsEndpoint_S3Settings {
            add_column_name: core::default::Default::default(),
            bucket_folder: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            canned_acl_for_objects: core::default::Default::default(),
            cdc_inserts_and_updates: core::default::Default::default(),
            cdc_inserts_only: core::default::Default::default(),
            cdc_max_batch_interval: core::default::Default::default(),
            cdc_min_file_size: core::default::Default::default(),
            cdc_path: core::default::Default::default(),
            compression_type: core::default::Default::default(),
            csv_delimiter: core::default::Default::default(),
            csv_no_sup_value: core::default::Default::default(),
            csv_null_value: core::default::Default::default(),
            csv_row_delimiter: core::default::Default::default(),
            data_format: core::default::Default::default(),
            data_page_size: core::default::Default::default(),
            date_partition_delimiter: core::default::Default::default(),
            date_partition_enabled: core::default::Default::default(),
            date_partition_sequence: core::default::Default::default(),
            dict_page_size_limit: core::default::Default::default(),
            enable_statistics: core::default::Default::default(),
            encoding_type: core::default::Default::default(),
            encryption_mode: core::default::Default::default(),
            external_table_definition: core::default::Default::default(),
            glue_catalog_generation: core::default::Default::default(),
            ignore_header_rows: core::default::Default::default(),
            ignore_headers_row: core::default::Default::default(),
            include_op_for_full_load: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            parquet_timestamp_in_millisecond: core::default::Default::default(),
            parquet_version: core::default::Default::default(),
            preserve_transactions: core::default::Default::default(),
            rfc_4180: core::default::Default::default(),
            row_group_length: core::default::Default::default(),
            server_side_encryption_kms_key_id: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            timestamp_column_name: core::default::Default::default(),
            use_csv_no_sup_value: core::default::Default::default(),
            use_task_start_time_for_full_load_timestamp: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDmsEndpoint_S3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDmsEndpoint_S3SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDmsEndpoint_S3SettingsRef {
        DataDmsEndpoint_S3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDmsEndpoint_S3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.base))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.base))
    }

    #[doc = "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.base))
    }

    #[doc = "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }

    #[doc = "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.base))
    }

    #[doc = "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.base))
    }

    #[doc = "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_generation` after provisioning.\n"]
    pub fn glue_catalog_generation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_catalog_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_headers_row` after provisioning.\n"]
    pub fn ignore_headers_row(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_headers_row", self.base))
    }

    #[doc = "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.base))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parquet_timestamp_in_millisecond", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.base))
    }

    #[doc = "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.base))
    }

    #[doc = "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.base))
    }

    #[doc = "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_task_start_time_for_full_load_timestamp", self.base),
        )
    }
}
