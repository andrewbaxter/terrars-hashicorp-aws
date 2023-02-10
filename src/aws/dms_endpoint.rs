use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    endpoint_id: PrimField<String>,
    endpoint_type: PrimField<String>,
    engine_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_connection_attributes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_settings: Option<Vec<DmsEndpointElasticsearchSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_settings: Option<Vec<DmsEndpointKafkaSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_settings: Option<Vec<DmsEndpointKinesisSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_settings: Option<Vec<DmsEndpointMongodbSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_settings: Option<Vec<DmsEndpointRedisSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift_settings: Option<Vec<DmsEndpointRedshiftSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_settings: Option<Vec<DmsEndpointS3SettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DmsEndpointTimeoutsEl>,
    dynamic: DmsEndpointDynamic,
}

struct DmsEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DmsEndpointData>,
}

#[derive(Clone)]
pub struct DmsEndpoint(Rc<DmsEndpoint_>);

impl DmsEndpoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `database_name`.\n"]
    pub fn set_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_name = Some(v.into());
        self
    }

    #[doc= "Set the field `extra_connection_attributes`.\n"]
    pub fn set_extra_connection_attributes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extra_connection_attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\n"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc= "Set the field `secrets_manager_access_role_arn`.\n"]
    pub fn set_secrets_manager_access_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secrets_manager_access_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secrets_manager_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `server_name`.\n"]
    pub fn set_server_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_name = Some(v.into());
        self
    }

    #[doc= "Set the field `service_access_role`.\n"]
    pub fn set_service_access_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_access_role = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_settings`.\n"]
    pub fn set_elasticsearch_settings(self, v: impl Into<BlockAssignable<DmsEndpointElasticsearchSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kafka_settings`.\n"]
    pub fn set_kafka_settings(self, v: impl Into<BlockAssignable<DmsEndpointKafkaSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kinesis_settings`.\n"]
    pub fn set_kinesis_settings(self, v: impl Into<BlockAssignable<DmsEndpointKinesisSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `mongodb_settings`.\n"]
    pub fn set_mongodb_settings(self, v: impl Into<BlockAssignable<DmsEndpointMongodbSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redis_settings`.\n"]
    pub fn set_redis_settings(self, v: impl Into<BlockAssignable<DmsEndpointRedisSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redis_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redis_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redshift_settings`.\n"]
    pub fn set_redshift_settings(self, v: impl Into<BlockAssignable<DmsEndpointRedshiftSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `s3_settings`.\n"]
    pub fn set_s3_settings(self, v: impl Into<BlockAssignable<DmsEndpointS3SettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DmsEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_connection_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_access_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> ListRef<DmsEndpointElasticsearchSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> ListRef<DmsEndpointKafkaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> ListRef<DmsEndpointKinesisSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> ListRef<DmsEndpointMongodbSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> ListRef<DmsEndpointRedisSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> ListRef<DmsEndpointRedshiftSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_settings` after provisioning.\n"]
    pub fn s3_settings(&self) -> ListRef<DmsEndpointS3SettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for DmsEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DmsEndpoint {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for DmsEndpoint {
    type O = ListRef<DmsEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for DmsEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_dms_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsEndpoint {
    pub tf_id: String,
    #[doc= ""]
    pub endpoint_id: PrimField<String>,
    #[doc= ""]
    pub endpoint_type: PrimField<String>,
    #[doc= ""]
    pub engine_name: PrimField<String>,
}

impl BuildDmsEndpoint {
    pub fn build(self, stack: &mut Stack) -> DmsEndpoint {
        let out = DmsEndpoint(Rc::new(DmsEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DmsEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_arn: core::default::Default::default(),
                database_name: core::default::Default::default(),
                endpoint_id: self.endpoint_id,
                endpoint_type: self.endpoint_type,
                engine_name: self.engine_name,
                extra_connection_attributes: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                password: core::default::Default::default(),
                port: core::default::Default::default(),
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
                redis_settings: core::default::Default::default(),
                redshift_settings: core::default::Default::default(),
                s3_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_name` after provisioning.\n"]
    pub fn engine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extra_connection_attributes` after provisioning.\n"]
    pub fn extra_connection_attributes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_connection_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_access_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_access_role` after provisioning.\n"]
    pub fn service_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_settings` after provisioning.\n"]
    pub fn elasticsearch_settings(&self) -> ListRef<DmsEndpointElasticsearchSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kafka_settings` after provisioning.\n"]
    pub fn kafka_settings(&self) -> ListRef<DmsEndpointKafkaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kafka_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kinesis_settings` after provisioning.\n"]
    pub fn kinesis_settings(&self) -> ListRef<DmsEndpointKinesisSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mongodb_settings` after provisioning.\n"]
    pub fn mongodb_settings(&self) -> ListRef<DmsEndpointMongodbSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mongodb_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redis_settings` after provisioning.\n"]
    pub fn redis_settings(&self) -> ListRef<DmsEndpointRedisSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redis_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> ListRef<DmsEndpointRedshiftSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_settings` after provisioning.\n"]
    pub fn s3_settings(&self) -> ListRef<DmsEndpointS3SettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointElasticsearchSettingsEl {
    endpoint_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_retry_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_load_error_percentage: Option<PrimField<f64>>,
    service_access_role_arn: PrimField<String>,
}

impl DmsEndpointElasticsearchSettingsEl {
    #[doc= "Set the field `error_retry_duration`.\n"]
    pub fn set_error_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.error_retry_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `full_load_error_percentage`.\n"]
    pub fn set_full_load_error_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.full_load_error_percentage = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointElasticsearchSettingsEl {
    type O = BlockAssignable<DmsEndpointElasticsearchSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointElasticsearchSettingsEl {
    #[doc= ""]
    pub endpoint_uri: PrimField<String>,
    #[doc= ""]
    pub service_access_role_arn: PrimField<String>,
}

impl BuildDmsEndpointElasticsearchSettingsEl {
    pub fn build(self) -> DmsEndpointElasticsearchSettingsEl {
        DmsEndpointElasticsearchSettingsEl {
            endpoint_uri: self.endpoint_uri,
            error_retry_duration: core::default::Default::default(),
            full_load_error_percentage: core::default::Default::default(),
            service_access_role_arn: self.service_access_role_arn,
        }
    }
}

pub struct DmsEndpointElasticsearchSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointElasticsearchSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointElasticsearchSettingsElRef {
        DmsEndpointElasticsearchSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointElasticsearchSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint_uri` after provisioning.\n"]
    pub fn endpoint_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `error_retry_duration` after provisioning.\n"]
    pub fn error_retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_retry_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `full_load_error_percentage` after provisioning.\n"]
    pub fn full_load_error_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_load_error_percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointKafkaSettingsEl {
    broker: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_control_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_null_and_empty: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_partition_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_table_alter_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_transaction_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_max_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_hex_prefix: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_include_schema_table: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_client_key_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}

impl DmsEndpointKafkaSettingsEl {
    #[doc= "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_control_details = Some(v.into());
        self
    }

    #[doc= "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_null_and_empty = Some(v.into());
        self
    }

    #[doc= "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_partition_value = Some(v.into());
        self
    }

    #[doc= "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_table_alter_operations = Some(v.into());
        self
    }

    #[doc= "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_transaction_details = Some(v.into());
        self
    }

    #[doc= "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }

    #[doc= "Set the field `message_max_bytes`.\n"]
    pub fn set_message_max_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.message_max_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `no_hex_prefix`.\n"]
    pub fn set_no_hex_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_hex_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.partition_include_schema_table = Some(v.into());
        self
    }

    #[doc= "Set the field `sasl_password`.\n"]
    pub fn set_sasl_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_password = Some(v.into());
        self
    }

    #[doc= "Set the field `sasl_username`.\n"]
    pub fn set_sasl_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_username = Some(v.into());
        self
    }

    #[doc= "Set the field `security_protocol`.\n"]
    pub fn set_security_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_client_certificate_arn`.\n"]
    pub fn set_ssl_client_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_certificate_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_client_key_arn`.\n"]
    pub fn set_ssl_client_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_client_key_password`.\n"]
    pub fn set_ssl_client_key_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_client_key_password = Some(v.into());
        self
    }

    #[doc= "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointKafkaSettingsEl {
    type O = BlockAssignable<DmsEndpointKafkaSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointKafkaSettingsEl {
    #[doc= ""]
    pub broker: PrimField<String>,
}

impl BuildDmsEndpointKafkaSettingsEl {
    pub fn build(self) -> DmsEndpointKafkaSettingsEl {
        DmsEndpointKafkaSettingsEl {
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

pub struct DmsEndpointKafkaSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointKafkaSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointKafkaSettingsElRef {
        DmsEndpointKafkaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointKafkaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `broker` after provisioning.\n"]
    pub fn broker(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.broker", self.base))
    }

    #[doc= "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_control_details", self.base))
    }

    #[doc= "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_null_and_empty", self.base))
    }

    #[doc= "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_partition_value", self.base))
    }

    #[doc= "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_table_alter_operations", self.base))
    }

    #[doc= "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_transaction_details", self.base))
    }

    #[doc= "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc= "Get a reference to the value of field `message_max_bytes` after provisioning.\n"]
    pub fn message_max_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_max_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `no_hex_prefix` after provisioning.\n"]
    pub fn no_hex_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_hex_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition_include_schema_table", self.base))
    }

    #[doc= "Get a reference to the value of field `sasl_password` after provisioning.\n"]
    pub fn sasl_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sasl_password", self.base))
    }

    #[doc= "Get a reference to the value of field `sasl_username` after provisioning.\n"]
    pub fn sasl_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sasl_username", self.base))
    }

    #[doc= "Get a reference to the value of field `security_protocol` after provisioning.\n"]
    pub fn security_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_client_certificate_arn` after provisioning.\n"]
    pub fn ssl_client_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_certificate_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_client_key_arn` after provisioning.\n"]
    pub fn ssl_client_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_client_key_password` after provisioning.\n"]
    pub fn ssl_client_key_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_key_password", self.base))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointKinesisSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_control_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_null_and_empty: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_partition_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_table_alter_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_transaction_details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_include_schema_table: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
}

impl DmsEndpointKinesisSettingsEl {
    #[doc= "Set the field `include_control_details`.\n"]
    pub fn set_include_control_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_control_details = Some(v.into());
        self
    }

    #[doc= "Set the field `include_null_and_empty`.\n"]
    pub fn set_include_null_and_empty(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_null_and_empty = Some(v.into());
        self
    }

    #[doc= "Set the field `include_partition_value`.\n"]
    pub fn set_include_partition_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_partition_value = Some(v.into());
        self
    }

    #[doc= "Set the field `include_table_alter_operations`.\n"]
    pub fn set_include_table_alter_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_table_alter_operations = Some(v.into());
        self
    }

    #[doc= "Set the field `include_transaction_details`.\n"]
    pub fn set_include_transaction_details(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_transaction_details = Some(v.into());
        self
    }

    #[doc= "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }

    #[doc= "Set the field `partition_include_schema_table`.\n"]
    pub fn set_partition_include_schema_table(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.partition_include_schema_table = Some(v.into());
        self
    }

    #[doc= "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointKinesisSettingsEl {
    type O = BlockAssignable<DmsEndpointKinesisSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointKinesisSettingsEl {}

impl BuildDmsEndpointKinesisSettingsEl {
    pub fn build(self) -> DmsEndpointKinesisSettingsEl {
        DmsEndpointKinesisSettingsEl {
            include_control_details: core::default::Default::default(),
            include_null_and_empty: core::default::Default::default(),
            include_partition_value: core::default::Default::default(),
            include_table_alter_operations: core::default::Default::default(),
            include_transaction_details: core::default::Default::default(),
            message_format: core::default::Default::default(),
            partition_include_schema_table: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
        }
    }
}

pub struct DmsEndpointKinesisSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointKinesisSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointKinesisSettingsElRef {
        DmsEndpointKinesisSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointKinesisSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `include_control_details` after provisioning.\n"]
    pub fn include_control_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_control_details", self.base))
    }

    #[doc= "Get a reference to the value of field `include_null_and_empty` after provisioning.\n"]
    pub fn include_null_and_empty(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_null_and_empty", self.base))
    }

    #[doc= "Get a reference to the value of field `include_partition_value` after provisioning.\n"]
    pub fn include_partition_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_partition_value", self.base))
    }

    #[doc= "Get a reference to the value of field `include_table_alter_operations` after provisioning.\n"]
    pub fn include_table_alter_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_table_alter_operations", self.base))
    }

    #[doc= "Get a reference to the value of field `include_transaction_details` after provisioning.\n"]
    pub fn include_transaction_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_transaction_details", self.base))
    }

    #[doc= "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc= "Get a reference to the value of field `partition_include_schema_table` after provisioning.\n"]
    pub fn partition_include_schema_table(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition_include_schema_table", self.base))
    }

    #[doc= "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointMongodbSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_mechanism: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docs_to_investigate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extract_doc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nesting_level: Option<PrimField<String>>,
}

impl DmsEndpointMongodbSettingsEl {
    #[doc= "Set the field `auth_mechanism`.\n"]
    pub fn set_auth_mechanism(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_mechanism = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_source`.\n"]
    pub fn set_auth_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_source = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_type`.\n"]
    pub fn set_auth_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_type = Some(v.into());
        self
    }

    #[doc= "Set the field `docs_to_investigate`.\n"]
    pub fn set_docs_to_investigate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.docs_to_investigate = Some(v.into());
        self
    }

    #[doc= "Set the field `extract_doc_id`.\n"]
    pub fn set_extract_doc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.extract_doc_id = Some(v.into());
        self
    }

    #[doc= "Set the field `nesting_level`.\n"]
    pub fn set_nesting_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nesting_level = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointMongodbSettingsEl {
    type O = BlockAssignable<DmsEndpointMongodbSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointMongodbSettingsEl {}

impl BuildDmsEndpointMongodbSettingsEl {
    pub fn build(self) -> DmsEndpointMongodbSettingsEl {
        DmsEndpointMongodbSettingsEl {
            auth_mechanism: core::default::Default::default(),
            auth_source: core::default::Default::default(),
            auth_type: core::default::Default::default(),
            docs_to_investigate: core::default::Default::default(),
            extract_doc_id: core::default::Default::default(),
            nesting_level: core::default::Default::default(),
        }
    }
}

pub struct DmsEndpointMongodbSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointMongodbSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointMongodbSettingsElRef {
        DmsEndpointMongodbSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointMongodbSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auth_mechanism` after provisioning.\n"]
    pub fn auth_mechanism(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_mechanism", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_source` after provisioning.\n"]
    pub fn auth_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_source", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc= "Get a reference to the value of field `docs_to_investigate` after provisioning.\n"]
    pub fn docs_to_investigate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.docs_to_investigate", self.base))
    }

    #[doc= "Get a reference to the value of field `extract_doc_id` after provisioning.\n"]
    pub fn extract_doc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extract_doc_id", self.base))
    }

    #[doc= "Get a reference to the value of field `nesting_level` after provisioning.\n"]
    pub fn nesting_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nesting_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointRedisSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_password: Option<PrimField<String>>,
    auth_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_user_name: Option<PrimField<String>>,
    port: PrimField<f64>,
    server_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_security_protocol: Option<PrimField<String>>,
}

impl DmsEndpointRedisSettingsEl {
    #[doc= "Set the field `auth_password`.\n"]
    pub fn set_auth_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_password = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_user_name`.\n"]
    pub fn set_auth_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_user_name = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_ca_certificate_arn`.\n"]
    pub fn set_ssl_ca_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_security_protocol`.\n"]
    pub fn set_ssl_security_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_security_protocol = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointRedisSettingsEl {
    type O = BlockAssignable<DmsEndpointRedisSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointRedisSettingsEl {
    #[doc= ""]
    pub auth_type: PrimField<String>,
    #[doc= ""]
    pub port: PrimField<f64>,
    #[doc= ""]
    pub server_name: PrimField<String>,
}

impl BuildDmsEndpointRedisSettingsEl {
    pub fn build(self) -> DmsEndpointRedisSettingsEl {
        DmsEndpointRedisSettingsEl {
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

pub struct DmsEndpointRedisSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointRedisSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointRedisSettingsElRef {
        DmsEndpointRedisSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointRedisSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auth_password` after provisioning.\n"]
    pub fn auth_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_password", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_user_name` after provisioning.\n"]
    pub fn auth_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_user_name", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_ca_certificate_arn` after provisioning.\n"]
    pub fn ssl_ca_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_certificate_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl_security_protocol` after provisioning.\n"]
    pub fn ssl_security_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_security_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointRedshiftSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
}

impl DmsEndpointRedshiftSettingsEl {
    #[doc= "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_folder = Some(v.into());
        self
    }

    #[doc= "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_side_encryption_kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointRedshiftSettingsEl {
    type O = BlockAssignable<DmsEndpointRedshiftSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointRedshiftSettingsEl {}

impl BuildDmsEndpointRedshiftSettingsEl {
    pub fn build(self) -> DmsEndpointRedshiftSettingsEl {
        DmsEndpointRedshiftSettingsEl {
            bucket_folder: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            encryption_mode: core::default::Default::default(),
            server_side_encryption_kms_key_id: core::default::Default::default(),
            service_access_role_arn: core::default::Default::default(),
        }
    }
}

pub struct DmsEndpointRedshiftSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointRedshiftSettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointRedshiftSettingsElRef {
        DmsEndpointRedshiftSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointRedshiftSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.base))
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointS3SettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_column_name: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl_for_objects: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_inserts_and_updates: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_inserts_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_max_batch_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_min_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_no_sup_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_null_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_row_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_page_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_sequence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dict_page_size_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_statistics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_table_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_header_rows: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_headers_row: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_op_for_full_load: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_timestamp_in_millisecond: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_transactions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rfc_4180: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_group_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_csv_no_sup_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_task_start_time_for_full_load_timestamp: Option<PrimField<bool>>,
}

impl DmsEndpointS3SettingsEl {
    #[doc= "Set the field `add_column_name`.\n"]
    pub fn set_add_column_name(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.add_column_name = Some(v.into());
        self
    }

    #[doc= "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_folder = Some(v.into());
        self
    }

    #[doc= "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc= "Set the field `canned_acl_for_objects`.\n"]
    pub fn set_canned_acl_for_objects(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.canned_acl_for_objects = Some(v.into());
        self
    }

    #[doc= "Set the field `cdc_inserts_and_updates`.\n"]
    pub fn set_cdc_inserts_and_updates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cdc_inserts_and_updates = Some(v.into());
        self
    }

    #[doc= "Set the field `cdc_inserts_only`.\n"]
    pub fn set_cdc_inserts_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cdc_inserts_only = Some(v.into());
        self
    }

    #[doc= "Set the field `cdc_max_batch_interval`.\n"]
    pub fn set_cdc_max_batch_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cdc_max_batch_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `cdc_min_file_size`.\n"]
    pub fn set_cdc_min_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cdc_min_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `cdc_path`.\n"]
    pub fn set_cdc_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cdc_path = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_type = Some(v.into());
        self
    }

    #[doc= "Set the field `csv_delimiter`.\n"]
    pub fn set_csv_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.csv_delimiter = Some(v.into());
        self
    }

    #[doc= "Set the field `csv_no_sup_value`.\n"]
    pub fn set_csv_no_sup_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.csv_no_sup_value = Some(v.into());
        self
    }

    #[doc= "Set the field `csv_null_value`.\n"]
    pub fn set_csv_null_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.csv_null_value = Some(v.into());
        self
    }

    #[doc= "Set the field `csv_row_delimiter`.\n"]
    pub fn set_csv_row_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.csv_row_delimiter = Some(v.into());
        self
    }

    #[doc= "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_format = Some(v.into());
        self
    }

    #[doc= "Set the field `data_page_size`.\n"]
    pub fn set_data_page_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_page_size = Some(v.into());
        self
    }

    #[doc= "Set the field `date_partition_delimiter`.\n"]
    pub fn set_date_partition_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date_partition_delimiter = Some(v.into());
        self
    }

    #[doc= "Set the field `date_partition_enabled`.\n"]
    pub fn set_date_partition_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.date_partition_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `date_partition_sequence`.\n"]
    pub fn set_date_partition_sequence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date_partition_sequence = Some(v.into());
        self
    }

    #[doc= "Set the field `dict_page_size_limit`.\n"]
    pub fn set_dict_page_size_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dict_page_size_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_statistics`.\n"]
    pub fn set_enable_statistics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_statistics = Some(v.into());
        self
    }

    #[doc= "Set the field `encoding_type`.\n"]
    pub fn set_encoding_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding_type = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `external_table_definition`.\n"]
    pub fn set_external_table_definition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_table_definition = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_header_rows`.\n"]
    pub fn set_ignore_header_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ignore_header_rows = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_headers_row`.\nThis setting has no effect, is deprecated, and will be removed in a future version"]
    pub fn set_ignore_headers_row(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ignore_headers_row = Some(v.into());
        self
    }

    #[doc= "Set the field `include_op_for_full_load`.\n"]
    pub fn set_include_op_for_full_load(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_op_for_full_load = Some(v.into());
        self
    }

    #[doc= "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `parquet_timestamp_in_millisecond`.\n"]
    pub fn set_parquet_timestamp_in_millisecond(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.parquet_timestamp_in_millisecond = Some(v.into());
        self
    }

    #[doc= "Set the field `parquet_version`.\n"]
    pub fn set_parquet_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parquet_version = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_transactions`.\n"]
    pub fn set_preserve_transactions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_transactions = Some(v.into());
        self
    }

    #[doc= "Set the field `rfc_4180`.\n"]
    pub fn set_rfc_4180(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rfc_4180 = Some(v.into());
        self
    }

    #[doc= "Set the field `row_group_length`.\n"]
    pub fn set_row_group_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.row_group_length = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_side_encryption_kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `service_access_role_arn`.\n"]
    pub fn set_service_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `timestamp_column_name`.\n"]
    pub fn set_timestamp_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp_column_name = Some(v.into());
        self
    }

    #[doc= "Set the field `use_csv_no_sup_value`.\n"]
    pub fn set_use_csv_no_sup_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_csv_no_sup_value = Some(v.into());
        self
    }

    #[doc= "Set the field `use_task_start_time_for_full_load_timestamp`.\n"]
    pub fn set_use_task_start_time_for_full_load_timestamp(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_task_start_time_for_full_load_timestamp = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointS3SettingsEl {
    type O = BlockAssignable<DmsEndpointS3SettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointS3SettingsEl {}

impl BuildDmsEndpointS3SettingsEl {
    pub fn build(self) -> DmsEndpointS3SettingsEl {
        DmsEndpointS3SettingsEl {
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

pub struct DmsEndpointS3SettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointS3SettingsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointS3SettingsElRef {
        DmsEndpointS3SettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointS3SettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.base))
    }

    #[doc= "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.base))
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.base))
    }

    #[doc= "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.base))
    }

    #[doc= "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.base))
    }

    #[doc= "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.base))
    }

    #[doc= "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc= "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.base))
    }

    #[doc= "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.base))
    }

    #[doc= "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.base))
    }

    #[doc= "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.base))
    }

    #[doc= "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }

    #[doc= "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.base))
    }

    #[doc= "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.base))
    }

    #[doc= "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.base))
    }

    #[doc= "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.base))
    }

    #[doc= "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.base))
    }

    #[doc= "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_headers_row` after provisioning.\nThis setting has no effect, is deprecated, and will be removed in a future version"]
    pub fn ignore_headers_row(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_headers_row", self.base))
    }

    #[doc= "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.base))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc= "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_timestamp_in_millisecond", self.base))
    }

    #[doc= "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.base))
    }

    #[doc= "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.base))
    }

    #[doc= "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.base))
    }

    #[doc= "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.base))
    }

    #[doc= "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_task_start_time_for_full_load_timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl DmsEndpointTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for DmsEndpointTimeoutsEl {
    type O = BlockAssignable<DmsEndpointTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsEndpointTimeoutsEl {}

impl BuildDmsEndpointTimeoutsEl {
    pub fn build(self) -> DmsEndpointTimeoutsEl {
        DmsEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DmsEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DmsEndpointTimeoutsElRef {
        DmsEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsEndpointTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct DmsEndpointDynamic {
    elasticsearch_settings: Option<DynamicBlock<DmsEndpointElasticsearchSettingsEl>>,
    kafka_settings: Option<DynamicBlock<DmsEndpointKafkaSettingsEl>>,
    kinesis_settings: Option<DynamicBlock<DmsEndpointKinesisSettingsEl>>,
    mongodb_settings: Option<DynamicBlock<DmsEndpointMongodbSettingsEl>>,
    redis_settings: Option<DynamicBlock<DmsEndpointRedisSettingsEl>>,
    redshift_settings: Option<DynamicBlock<DmsEndpointRedshiftSettingsEl>>,
    s3_settings: Option<DynamicBlock<DmsEndpointS3SettingsEl>>,
}
