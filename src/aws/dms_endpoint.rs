#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsEndpointData_ {
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
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    endpoint_id: terrars::PrimField<std::string::String>,
    endpoint_type: terrars::PrimField<std::string::String>,
    engine_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_connection_attributes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pause_replication_tasks: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_settings: std::option::Option<std::vec::Vec<DmsEndpoint_ElasticsearchSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_settings: std::option::Option<std::vec::Vec<DmsEndpoint_KafkaSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_settings: std::option::Option<std::vec::Vec<DmsEndpoint_KinesisSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_settings: std::option::Option<std::vec::Vec<DmsEndpoint_MongodbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_settings: std::option::Option<std::vec::Vec<DmsEndpoint_MysqlSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_settings: std::option::Option<std::vec::Vec<DmsEndpoint_OracleSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgres_settings: std::option::Option<std::vec::Vec<DmsEndpoint_PostgresSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_settings: std::option::Option<std::vec::Vec<DmsEndpoint_RedisSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_settings: std::option::Option<std::vec::Vec<DmsEndpoint_RedshiftSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DmsEndpoint_Timeouts>,
    dynamic: DmsEndpointDynamic,
}

struct DmsEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DmsEndpointData_>,
}

#[derive(Clone)]
pub struct DmsEndpoint(std::rc::Rc<DmsEndpoint_>);

impl DmsEndpoint {
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

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_connection_attributes`.\n"]
    pub fn set_extra_connection_attributes(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().extra_connection_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pause_replication_tasks`.\n"]
    pub fn set_pause_replication_tasks(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pause_replication_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_access_role_arn`.\n"]
    pub fn set_secrets_manager_access_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().secrets_manager_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().secrets_manager_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_role`.\n"]
    pub fn set_service_access_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_access_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elasticsearch_settings`.\n"]
    pub fn set_elasticsearch_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_ElasticsearchSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_settings`.\n"]
    pub fn set_kafka_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_KafkaSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_settings`.\n"]
    pub fn set_kinesis_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_KinesisSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mongodb_settings`.\n"]
    pub fn set_mongodb_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_MongodbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_settings`.\n"]
    pub fn set_mysql_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_MysqlSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_settings`.\n"]
    pub fn set_oracle_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_OracleSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgres_settings`.\n"]
    pub fn set_postgres_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_PostgresSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgres_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgres_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redis_settings`.\n"]
    pub fn set_redis_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_RedisSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redis_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redis_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_settings`.\n"]
    pub fn set_redshift_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsEndpoint_RedshiftSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DmsEndpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pause_replication_tasks` after provisioning.\n"]
    pub fn pause_replication_tasks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_replication_tasks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> terrars::ListRef<DmsEndpoint_ElasticsearchSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> terrars::ListRef<DmsEndpoint_KafkaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> terrars::ListRef<DmsEndpoint_KinesisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> terrars::ListRef<DmsEndpoint_MongodbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> terrars::ListRef<DmsEndpoint_MysqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_settings` after provisioning.\n"]
    pub fn oracle_settings(&self) -> terrars::ListRef<DmsEndpoint_OracleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> terrars::ListRef<DmsEndpoint_PostgresSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> terrars::ListRef<DmsEndpoint_RedisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> terrars::ListRef<DmsEndpoint_RedshiftSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpoint_TimeoutsRef {
        DmsEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DmsEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DmsEndpoint { }

impl terrars::ToListMappable for DmsEndpoint {
    type O = terrars::ListRef<DmsEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DmsEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dms_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub endpoint_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub engine_name: terrars::PrimField<std::string::String>,
}

impl BuildDmsEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DmsEndpoint {
        let out = DmsEndpoint(std::rc::Rc::new(DmsEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DmsEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                database_name: core::default::Default::default(),
                endpoint_id: self.endpoint_id,
                endpoint_type: self.endpoint_type,
                engine_name: self.engine_name,
                extra_connection_attributes: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                password: core::default::Default::default(),
                pause_replication_tasks: core::default::Default::default(),
                port: core::default::Default::default(),
                region: core::default::Default::default(),
                secrets_manager_access_role_arn: core::default::Default::default(),
                secrets_manager_arn: core::default::Default::default(),
                server_name: core::default::Default::default(),
                service_access_role: core::default::Default::default(),
                ssl_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                username: core::default::Default::default(),
                elasticsearch_settings: core::default::Default::default(),
                kafka_settings: core::default::Default::default(),
                kinesis_settings: core::default::Default::default(),
                mongodb_settings: core::default::Default::default(),
                mysql_settings: core::default::Default::default(),
                oracle_settings: core::default::Default::default(),
                postgres_settings: core::default::Default::default(),
                redis_settings: core::default::Default::default(),
                redshift_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsEndpointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pause_replication_tasks` after provisioning.\n"]
    pub fn pause_replication_tasks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_replication_tasks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> terrars::ListRef<DmsEndpoint_ElasticsearchSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> terrars::ListRef<DmsEndpoint_KafkaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> terrars::ListRef<DmsEndpoint_KinesisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> terrars::ListRef<DmsEndpoint_MongodbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> terrars::ListRef<DmsEndpoint_MysqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_settings` after provisioning.\n"]
    pub fn oracle_settings(&self) -> terrars::ListRef<DmsEndpoint_OracleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_settings` after provisioning.\n"]
    pub fn postgres_settings(&self) -> terrars::ListRef<DmsEndpoint_PostgresSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> terrars::ListRef<DmsEndpoint_RedisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> terrars::ListRef<DmsEndpoint_RedshiftSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpoint_TimeoutsRef {
        DmsEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DmsEndpointDynamic {
    elasticsearch_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_ElasticsearchSettings>>,
    kafka_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_KafkaSettings>>,
    kinesis_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_KinesisSettings>>,
    mongodb_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_MongodbSettings>>,
    mysql_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_MysqlSettings>>,
    oracle_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_OracleSettings>>,
    postgres_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_PostgresSettings>>,
    redis_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_RedisSettings>>,
    redshift_settings: std::option::Option<terrars::DynamicBlock<DmsEndpoint_RedshiftSettings>>,
}

#[derive(Serialize)]
pub struct DmsEndpoint_ElasticsearchSettings {
    endpoint_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_retry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_load_error_percentage: std::option::Option<terrars::PrimField<f64>>,
    service_access_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_new_mapping_type: std::option::Option<terrars::PrimField<bool>>,
}

impl DmsEndpoint_ElasticsearchSettings {
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

    #[doc = "Set the field `use_new_mapping_type`.\n"]
    pub fn set_use_new_mapping_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_new_mapping_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsEndpoint_ElasticsearchSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_ElasticsearchSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_ElasticsearchSettings {
    #[doc = ""]
    pub endpoint_uri: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_access_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsEndpoint_ElasticsearchSettings {
    pub fn build(self) -> DmsEndpoint_ElasticsearchSettings {
        DmsEndpoint_ElasticsearchSettings {
            endpoint_uri: self.endpoint_uri,
            error_retry_duration: core::default::Default::default(),
            full_load_error_percentage: core::default::Default::default(),
            service_access_role_arn: self.service_access_role_arn,
            use_new_mapping_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsEndpoint_ElasticsearchSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_ElasticsearchSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_ElasticsearchSettingsRef {
        DmsEndpoint_ElasticsearchSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_ElasticsearchSettingsRef {
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

    #[doc = "Get a reference to the value of field `use_new_mapping_type` after provisioning.\n"]
    pub fn use_new_mapping_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_new_mapping_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpoint_KafkaSettings {
    broker: terrars::PrimField<std::string::String>,
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

impl DmsEndpoint_KafkaSettings {
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

impl terrars::ToListMappable for DmsEndpoint_KafkaSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_KafkaSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_KafkaSettings {
    #[doc = ""]
    pub broker: terrars::PrimField<std::string::String>,
}

impl BuildDmsEndpoint_KafkaSettings {
    pub fn build(self) -> DmsEndpoint_KafkaSettings {
        DmsEndpoint_KafkaSettings {
            broker: self.broker,
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
pub struct DmsEndpoint_KafkaSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_KafkaSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_KafkaSettingsRef {
        DmsEndpoint_KafkaSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_KafkaSettingsRef {
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
pub struct DmsEndpoint_KinesisSettings {
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

impl DmsEndpoint_KinesisSettings {
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

impl terrars::ToListMappable for DmsEndpoint_KinesisSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_KinesisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_KinesisSettings {}

impl BuildDmsEndpoint_KinesisSettings {
    pub fn build(self) -> DmsEndpoint_KinesisSettings {
        DmsEndpoint_KinesisSettings {
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
pub struct DmsEndpoint_KinesisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_KinesisSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_KinesisSettingsRef {
        DmsEndpoint_KinesisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_KinesisSettingsRef {
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
pub struct DmsEndpoint_MongodbSettings {
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

impl DmsEndpoint_MongodbSettings {
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

impl terrars::ToListMappable for DmsEndpoint_MongodbSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_MongodbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_MongodbSettings {}

impl BuildDmsEndpoint_MongodbSettings {
    pub fn build(self) -> DmsEndpoint_MongodbSettings {
        DmsEndpoint_MongodbSettings {
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
pub struct DmsEndpoint_MongodbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_MongodbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_MongodbSettingsRef {
        DmsEndpoint_MongodbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_MongodbSettingsRef {
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
pub struct DmsEndpoint_MysqlSettings {
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

impl DmsEndpoint_MysqlSettings {
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

impl terrars::ToListMappable for DmsEndpoint_MysqlSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_MysqlSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_MysqlSettings {}

impl BuildDmsEndpoint_MysqlSettings {
    pub fn build(self) -> DmsEndpoint_MysqlSettings {
        DmsEndpoint_MysqlSettings {
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
pub struct DmsEndpoint_MysqlSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_MysqlSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_MysqlSettingsRef {
        DmsEndpoint_MysqlSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_MysqlSettingsRef {
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
pub struct DmsEndpoint_OracleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_alternate_directly: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_supplemental_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_archived_log_dest_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_selected_nested_tables: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archived_log_dest_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archived_logs_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asm_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asm_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asm_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    char_length_semantics: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_timestamp_with_zone_to_utc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_path_no_log: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_path_parallel_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_homogenous_tablespace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_archived_log_dest_ids: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_task_on_lob_truncation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_datatype_scale: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_transaction_window: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_path_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallel_asm_read_threads: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_ahead_blocks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_table_space_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_path_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_oracle_asm_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_oracle_asm_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_db_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_db_encryption_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spatial_data_option_to_geo_json_function_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_delay_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trim_space_in_char: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_alternate_folder_for_online: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_bfile: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_direct_path_full_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_logminer_reader: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_path_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsEndpoint_OracleSettings {
    #[doc = "Set the field `access_alternate_directly`.\n"]
    pub fn set_access_alternate_directly(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.access_alternate_directly = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_supplemental_logging`.\n"]
    pub fn set_add_supplemental_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.add_supplemental_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_archived_log_dest_id`.\n"]
    pub fn set_additional_archived_log_dest_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_archived_log_dest_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_selected_nested_tables`.\n"]
    pub fn set_allow_selected_nested_tables(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_selected_nested_tables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `archived_log_dest_id`.\n"]
    pub fn set_archived_log_dest_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.archived_log_dest_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `archived_logs_only`.\n"]
    pub fn set_archived_logs_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.archived_logs_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asm_password`.\n"]
    pub fn set_asm_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asm_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asm_server`.\n"]
    pub fn set_asm_server(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asm_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asm_user`.\n"]
    pub fn set_asm_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asm_user = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `char_length_semantics`.\n"]
    pub fn set_char_length_semantics(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.char_length_semantics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `convert_timestamp_with_zone_to_utc`.\n"]
    pub fn set_convert_timestamp_with_zone_to_utc(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.convert_timestamp_with_zone_to_utc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_path_no_log`.\n"]
    pub fn set_direct_path_no_log(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.direct_path_no_log = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_path_parallel_load`.\n"]
    pub fn set_direct_path_parallel_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.direct_path_parallel_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_homogenous_tablespace`.\n"]
    pub fn set_enable_homogenous_tablespace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_homogenous_tablespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_archived_log_dest_ids`.\n"]
    pub fn set_extra_archived_log_dest_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.extra_archived_log_dest_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_task_on_lob_truncation`.\n"]
    pub fn set_fail_task_on_lob_truncation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_task_on_lob_truncation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_datatype_scale`.\n"]
    pub fn set_number_datatype_scale(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_datatype_scale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_transaction_window`.\n"]
    pub fn set_open_transaction_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.open_transaction_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oracle_path_prefix`.\n"]
    pub fn set_oracle_path_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oracle_path_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallel_asm_read_threads`.\n"]
    pub fn set_parallel_asm_read_threads(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallel_asm_read_threads = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_ahead_blocks`.\n"]
    pub fn set_read_ahead_blocks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_ahead_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_table_space_name`.\n"]
    pub fn set_read_table_space_name(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_table_space_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replace_path_prefix`.\n"]
    pub fn set_replace_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace_path_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_interval`.\n"]
    pub fn set_retry_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_oracle_asm_access_role_arn`.\n"]
    pub fn set_secrets_manager_oracle_asm_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_oracle_asm_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_oracle_asm_secret_id`.\n"]
    pub fn set_secrets_manager_oracle_asm_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_oracle_asm_secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_db_encryption`.\n"]
    pub fn set_security_db_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_db_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_db_encryption_name`.\n"]
    pub fn set_security_db_encryption_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_db_encryption_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spatial_data_option_to_geo_json_function_name`.\n"]
    pub fn set_spatial_data_option_to_geo_json_function_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spatial_data_option_to_geo_json_function_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standby_delay_time`.\n"]
    pub fn set_standby_delay_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.standby_delay_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trim_space_in_char`.\n"]
    pub fn set_trim_space_in_char(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.trim_space_in_char = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_alternate_folder_for_online`.\n"]
    pub fn set_use_alternate_folder_for_online(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_alternate_folder_for_online = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_bfile`.\n"]
    pub fn set_use_bfile(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_bfile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_direct_path_full_load`.\n"]
    pub fn set_use_direct_path_full_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_direct_path_full_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_logminer_reader`.\n"]
    pub fn set_use_logminer_reader(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_logminer_reader = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_path_prefix`.\n"]
    pub fn set_use_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsEndpoint_OracleSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_OracleSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_OracleSettings {}

impl BuildDmsEndpoint_OracleSettings {
    pub fn build(self) -> DmsEndpoint_OracleSettings {
        DmsEndpoint_OracleSettings {
            access_alternate_directly: core::default::Default::default(),
            add_supplemental_logging: core::default::Default::default(),
            additional_archived_log_dest_id: core::default::Default::default(),
            allow_selected_nested_tables: core::default::Default::default(),
            archived_log_dest_id: core::default::Default::default(),
            archived_logs_only: core::default::Default::default(),
            asm_password: core::default::Default::default(),
            asm_server: core::default::Default::default(),
            asm_user: core::default::Default::default(),
            authentication_method: core::default::Default::default(),
            char_length_semantics: core::default::Default::default(),
            convert_timestamp_with_zone_to_utc: core::default::Default::default(),
            direct_path_no_log: core::default::Default::default(),
            direct_path_parallel_load: core::default::Default::default(),
            enable_homogenous_tablespace: core::default::Default::default(),
            extra_archived_log_dest_ids: core::default::Default::default(),
            fail_task_on_lob_truncation: core::default::Default::default(),
            number_datatype_scale: core::default::Default::default(),
            open_transaction_window: core::default::Default::default(),
            oracle_path_prefix: core::default::Default::default(),
            parallel_asm_read_threads: core::default::Default::default(),
            read_ahead_blocks: core::default::Default::default(),
            read_table_space_name: core::default::Default::default(),
            replace_path_prefix: core::default::Default::default(),
            retry_interval: core::default::Default::default(),
            secrets_manager_oracle_asm_access_role_arn: core::default::Default::default(),
            secrets_manager_oracle_asm_secret_id: core::default::Default::default(),
            security_db_encryption: core::default::Default::default(),
            security_db_encryption_name: core::default::Default::default(),
            spatial_data_option_to_geo_json_function_name: core::default::Default::default(),
            standby_delay_time: core::default::Default::default(),
            trim_space_in_char: core::default::Default::default(),
            use_alternate_folder_for_online: core::default::Default::default(),
            use_bfile: core::default::Default::default(),
            use_direct_path_full_load: core::default::Default::default(),
            use_logminer_reader: core::default::Default::default(),
            use_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsEndpoint_OracleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_OracleSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_OracleSettingsRef {
        DmsEndpoint_OracleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_OracleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_alternate_directly` after provisioning.\n"]
    pub fn access_alternate_directly(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_alternate_directly", self.base))
    }

    #[doc = "Get a reference to the value of field `add_supplemental_logging` after provisioning.\n"]
    pub fn add_supplemental_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.add_supplemental_logging", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_archived_log_dest_id` after provisioning.\n"]
    pub fn additional_archived_log_dest_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_archived_log_dest_id", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_selected_nested_tables` after provisioning.\n"]
    pub fn allow_selected_nested_tables(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_selected_nested_tables", self.base))
    }

    #[doc = "Get a reference to the value of field `archived_log_dest_id` after provisioning.\n"]
    pub fn archived_log_dest_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archived_log_dest_id", self.base))
    }

    #[doc = "Get a reference to the value of field `archived_logs_only` after provisioning.\n"]
    pub fn archived_logs_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archived_logs_only", self.base))
    }

    #[doc = "Get a reference to the value of field `asm_password` after provisioning.\n"]
    pub fn asm_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asm_password", self.base))
    }

    #[doc = "Get a reference to the value of field `asm_server` after provisioning.\n"]
    pub fn asm_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asm_server", self.base))
    }

    #[doc = "Get a reference to the value of field `asm_user` after provisioning.\n"]
    pub fn asm_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asm_user", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_method", self.base))
    }

    #[doc = "Get a reference to the value of field `char_length_semantics` after provisioning.\n"]
    pub fn char_length_semantics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.char_length_semantics", self.base))
    }

    #[doc = "Get a reference to the value of field `convert_timestamp_with_zone_to_utc` after provisioning.\n"]
    pub fn convert_timestamp_with_zone_to_utc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.convert_timestamp_with_zone_to_utc", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_path_no_log` after provisioning.\n"]
    pub fn direct_path_no_log(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_path_no_log", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_path_parallel_load` after provisioning.\n"]
    pub fn direct_path_parallel_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_path_parallel_load", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_homogenous_tablespace` after provisioning.\n"]
    pub fn enable_homogenous_tablespace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_homogenous_tablespace", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_archived_log_dest_ids` after provisioning.\n"]
    pub fn extra_archived_log_dest_ids(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_archived_log_dest_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_task_on_lob_truncation` after provisioning.\n"]
    pub fn fail_task_on_lob_truncation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_task_on_lob_truncation", self.base))
    }

    #[doc = "Get a reference to the value of field `number_datatype_scale` after provisioning.\n"]
    pub fn number_datatype_scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_datatype_scale", self.base))
    }

    #[doc = "Get a reference to the value of field `open_transaction_window` after provisioning.\n"]
    pub fn open_transaction_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_transaction_window", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_path_prefix` after provisioning.\n"]
    pub fn oracle_path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oracle_path_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `parallel_asm_read_threads` after provisioning.\n"]
    pub fn parallel_asm_read_threads(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallel_asm_read_threads", self.base))
    }

    #[doc = "Get a reference to the value of field `read_ahead_blocks` after provisioning.\n"]
    pub fn read_ahead_blocks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_ahead_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `read_table_space_name` after provisioning.\n"]
    pub fn read_table_space_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_table_space_name", self.base))
    }

    #[doc = "Get a reference to the value of field `replace_path_prefix` after provisioning.\n"]
    pub fn replace_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace_path_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_interval` after provisioning.\n"]
    pub fn retry_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_oracle_asm_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_oracle_asm_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_oracle_asm_access_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `secrets_manager_oracle_asm_secret_id` after provisioning.\n"]
    pub fn secrets_manager_oracle_asm_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_oracle_asm_secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `security_db_encryption` after provisioning.\n"]
    pub fn security_db_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_db_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `security_db_encryption_name` after provisioning.\n"]
    pub fn security_db_encryption_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_db_encryption_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spatial_data_option_to_geo_json_function_name` after provisioning.\n"]
    pub fn spatial_data_option_to_geo_json_function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.spatial_data_option_to_geo_json_function_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `standby_delay_time` after provisioning.\n"]
    pub fn standby_delay_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_delay_time", self.base))
    }

    #[doc = "Get a reference to the value of field `trim_space_in_char` after provisioning.\n"]
    pub fn trim_space_in_char(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trim_space_in_char", self.base))
    }

    #[doc = "Get a reference to the value of field `use_alternate_folder_for_online` after provisioning.\n"]
    pub fn use_alternate_folder_for_online(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_alternate_folder_for_online", self.base))
    }

    #[doc = "Get a reference to the value of field `use_bfile` after provisioning.\n"]
    pub fn use_bfile(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_bfile", self.base))
    }

    #[doc = "Get a reference to the value of field `use_direct_path_full_load` after provisioning.\n"]
    pub fn use_direct_path_full_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_direct_path_full_load", self.base))
    }

    #[doc = "Get a reference to the value of field `use_logminer_reader` after provisioning.\n"]
    pub fn use_logminer_reader(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_logminer_reader", self.base))
    }

    #[doc = "Get a reference to the value of field `use_path_prefix` after provisioning.\n"]
    pub fn use_path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpoint_PostgresSettings {
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

impl DmsEndpoint_PostgresSettings {
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

impl terrars::ToListMappable for DmsEndpoint_PostgresSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_PostgresSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_PostgresSettings {}

impl BuildDmsEndpoint_PostgresSettings {
    pub fn build(self) -> DmsEndpoint_PostgresSettings {
        DmsEndpoint_PostgresSettings {
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
pub struct DmsEndpoint_PostgresSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_PostgresSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_PostgresSettingsRef {
        DmsEndpoint_PostgresSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_PostgresSettingsRef {
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
pub struct DmsEndpoint_RedisSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_password: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    port: terrars::PrimField<f64>,
    server_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsEndpoint_RedisSettings {
    #[doc = "Set the field `auth_password`.\n"]
    pub fn set_auth_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_user_name`.\n"]
    pub fn set_auth_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_user_name = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DmsEndpoint_RedisSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_RedisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_RedisSettings {
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub server_name: terrars::PrimField<std::string::String>,
}

impl BuildDmsEndpoint_RedisSettings {
    pub fn build(self) -> DmsEndpoint_RedisSettings {
        DmsEndpoint_RedisSettings {
            auth_password: core::default::Default::default(),
            auth_type: self.auth_type,
            auth_user_name: core::default::Default::default(),
            port: self.port,
            server_name: self.server_name,
            ssl_ca_certificate_arn: core::default::Default::default(),
            ssl_security_protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsEndpoint_RedisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_RedisSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_RedisSettingsRef {
        DmsEndpoint_RedisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_RedisSettingsRef {
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
pub struct DmsEndpoint_RedshiftSettings {
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

impl DmsEndpoint_RedshiftSettings {
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

impl terrars::ToListMappable for DmsEndpoint_RedshiftSettings {
    type O = terrars::BlockAssignable<DmsEndpoint_RedshiftSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_RedshiftSettings {}

impl BuildDmsEndpoint_RedshiftSettings {
    pub fn build(self) -> DmsEndpoint_RedshiftSettings {
        DmsEndpoint_RedshiftSettings {
            bucket_folder: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            encryption_mode: core::default::Default::default(),
            server_side_encryption_kms_key_id: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsEndpoint_RedshiftSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_RedshiftSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_RedshiftSettingsRef {
        DmsEndpoint_RedshiftSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_RedshiftSettingsRef {
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
pub struct DmsEndpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsEndpoint_Timeouts {
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
}

impl terrars::ToListMappable for DmsEndpoint_Timeouts {
    type O = terrars::BlockAssignable<DmsEndpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpoint_Timeouts {}

impl BuildDmsEndpoint_Timeouts {
    pub fn build(self) -> DmsEndpoint_Timeouts {
        DmsEndpoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsEndpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsEndpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsEndpoint_TimeoutsRef {
        DmsEndpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpoint_TimeoutsRef {
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
}
