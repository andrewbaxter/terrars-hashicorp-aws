use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KinesisFirehoseDeliveryStreamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_configuration: Option<Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_s3_configuration: Option<Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_endpoint_configuration: Option<Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_source_configuration: Option<Vec<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift_configuration: Option<Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration: Option<Vec<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<Vec<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splunk_configuration: Option<Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamDynamic,
}

struct KinesisFirehoseDeliveryStream_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KinesisFirehoseDeliveryStreamData>,
}

#[derive(Clone)]
pub struct KinesisFirehoseDeliveryStream(Rc<KinesisFirehoseDeliveryStream_>);

impl KinesisFirehoseDeliveryStream {
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

    #[doc= "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_id`.\n"]
    pub fn set_destination_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destination_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc= "Set the field `version_id`.\n"]
    pub fn set_version_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_id = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_configuration`.\n"]
    pub fn set_elasticsearch_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `extended_s3_configuration`.\n"]
    pub fn set_extended_s3_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extended_s3_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extended_s3_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `http_endpoint_configuration`.\n"]
    pub fn set_http_endpoint_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_endpoint_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_endpoint_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kinesis_source_configuration`.\n"]
    pub fn set_kinesis_source_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_source_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_source_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redshift_configuration`.\n"]
    pub fn set_redshift_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `splunk_configuration`.\n"]
    pub fn set_splunk_configuration(
        self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().splunk_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.splunk_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_id` after provisioning.\n"]
    pub fn destination_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_configuration` after provisioning.\n"]
    pub fn elasticsearch_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<KinesisFirehoseDeliveryStreamServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
    }
}

impl Resource for KinesisFirehoseDeliveryStream {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for KinesisFirehoseDeliveryStream {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStream {
    type O = ListRef<KinesisFirehoseDeliveryStreamRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KinesisFirehoseDeliveryStream_ {
    fn extract_resource_type(&self) -> String {
        "aws_kinesis_firehose_delivery_stream".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisFirehoseDeliveryStream {
    pub tf_id: String,
    #[doc= ""]
    pub destination: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStream {
    pub fn build(self, stack: &mut Stack) -> KinesisFirehoseDeliveryStream {
        let out = KinesisFirehoseDeliveryStream(Rc::new(KinesisFirehoseDeliveryStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KinesisFirehoseDeliveryStreamData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                arn: core::default::Default::default(),
                destination: self.destination,
                destination_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_id: core::default::Default::default(),
                elasticsearch_configuration: core::default::Default::default(),
                extended_s3_configuration: core::default::Default::default(),
                http_endpoint_configuration: core::default::Default::default(),
                kinesis_source_configuration: core::default::Default::default(),
                redshift_configuration: core::default::Default::default(),
                s3_configuration: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                splunk_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KinesisFirehoseDeliveryStreamRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KinesisFirehoseDeliveryStreamRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_id` after provisioning.\n"]
    pub fn destination_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_configuration` after provisioning.\n"]
    pub fn elasticsearch_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<KinesisFirehoseDeliveryStreamServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc= ""]
    pub parameter_name: PrimField<String>,
    #[doc= ""]
    pub parameter_value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElDynamic {
    processors: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processors: Option<
        Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    role_arn: PrimField<String>,
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    #[doc= ""]
    pub role_arn: PrimField<String>,
    #[doc= ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc= ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
            role_arn: self.role_arn,
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>,
    >,
    vpc_config: Option<DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buffering_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffering_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_arn: Option<PrimField<String>>,
    index_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_rotation_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_duration: Option<PrimField<f64>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<
        Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>>,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    #[doc= "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffering_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffering_size = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_endpoint`.\n"]
    pub fn set_cluster_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `domain_arn`.\n"]
    pub fn set_domain_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `index_rotation_period`.\n"]
    pub fn set_index_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_rotation_period = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_backup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `type_name`.\n"]
    pub fn set_type_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_name = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    #[doc= ""]
    pub index_name: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            cluster_endpoint: core::default::Default::default(),
            domain_arn: core::default::Default::default(),
            index_name: self.index_name,
            index_rotation_period: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            type_name: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_endpoint` after provisioning.\n"]
    pub fn cluster_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `domain_arn` after provisioning.\n"]
    pub fn domain_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc= "Get a reference to the value of field `index_rotation_period` after provisioning.\n"]
    pub fn index_rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_rotation_period", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc= "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_formats: Option<ListField<PrimField<String>>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    #[doc= "Set the field `timestamp_formats`.\n"]
    pub fn set_timestamp_formats(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.timestamp_formats = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
            timestamp_formats: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `timestamp_formats` after provisioning.\n"]
    pub fn timestamp_formats(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.timestamp_formats", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    case_insensitive: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_to_json_key_mappings: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_dots_in_json_keys_to_underscores: Option<PrimField<bool>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    #[doc= "Set the field `case_insensitive`.\n"]
    pub fn set_case_insensitive(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.case_insensitive = Some(v.into());
        self
    }

    #[doc= "Set the field `column_to_json_key_mappings`.\n"]
    pub fn set_column_to_json_key_mappings(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.column_to_json_key_mappings = Some(v.into());
        self
    }

    #[doc= "Set the field `convert_dots_in_json_keys_to_underscores`.\n"]
    pub fn set_convert_dots_in_json_keys_to_underscores(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.convert_dots_in_json_keys_to_underscores = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
            case_insensitive: core::default::Default::default(),
            column_to_json_key_mappings: core::default::Default::default(),
            convert_dots_in_json_keys_to_underscores: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `case_insensitive` after provisioning.\n"]
    pub fn case_insensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.case_insensitive", self.base))
    }

    #[doc= "Get a reference to the value of field `column_to_json_key_mappings` after provisioning.\n"]
    pub fn column_to_json_key_mappings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.column_to_json_key_mappings", self.base))
    }

    #[doc= "Get a reference to the value of field `convert_dots_in_json_keys_to_underscores` after provisioning.\n"]
    pub fn convert_dots_in_json_keys_to_underscores(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.convert_dots_in_json_keys_to_underscores", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElDynamic {
    hive_json_ser_de: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >,
    >,
    open_x_json_ser_de: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_json_ser_de: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_x_json_ser_de: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    #[doc= "Set the field `hive_json_ser_de`.\n"]
    pub fn set_hive_json_ser_de(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hive_json_ser_de = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hive_json_ser_de = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `open_x_json_ser_de`.\n"]
    pub fn set_open_x_json_ser_de(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.open_x_json_ser_de = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.open_x_json_ser_de = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
            hive_json_ser_de: core::default::Default::default(),
            open_x_json_ser_de: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hive_json_ser_de` after provisioning.\n"]
    pub fn hive_json_ser_de(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.hive_json_ser_de", self.base))
    }

    #[doc= "Get a reference to the value of field `open_x_json_ser_de` after provisioning.\n"]
    pub fn open_x_json_ser_de(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.open_x_json_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDynamic {
    deserializer: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deserializer: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    #[doc= "Set the field `deserializer`.\n"]
    pub fn set_deserializer(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deserializer = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deserializer = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
            deserializer: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deserializer` after provisioning.\n"]
    pub fn deserializer(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.deserializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bloom_filter_columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bloom_filter_false_positive_probability: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary_key_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_padding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding_tolerance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_index_stride: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stripe_size_bytes: Option<PrimField<f64>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
    #[doc= "Set the field `block_size_bytes`.\n"]
    pub fn set_block_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_size_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `bloom_filter_columns`.\n"]
    pub fn set_bloom_filter_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bloom_filter_columns = Some(v.into());
        self
    }

    #[doc= "Set the field `bloom_filter_false_positive_probability`.\n"]
    pub fn set_bloom_filter_false_positive_probability(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bloom_filter_false_positive_probability = Some(v.into());
        self
    }

    #[doc= "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }

    #[doc= "Set the field `dictionary_key_threshold`.\n"]
    pub fn set_dictionary_key_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dictionary_key_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_padding`.\n"]
    pub fn set_enable_padding(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_padding = Some(v.into());
        self
    }

    #[doc= "Set the field `format_version`.\n"]
    pub fn set_format_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format_version = Some(v.into());
        self
    }

    #[doc= "Set the field `padding_tolerance`.\n"]
    pub fn set_padding_tolerance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.padding_tolerance = Some(v.into());
        self
    }

    #[doc= "Set the field `row_index_stride`.\n"]
    pub fn set_row_index_stride(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.row_index_stride = Some(v.into());
        self
    }

    #[doc= "Set the field `stripe_size_bytes`.\n"]
    pub fn set_stripe_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.stripe_size_bytes = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
            block_size_bytes: core::default::Default::default(),
            bloom_filter_columns: core::default::Default::default(),
            bloom_filter_false_positive_probability: core::default::Default::default(),
            compression: core::default::Default::default(),
            dictionary_key_threshold: core::default::Default::default(),
            enable_padding: core::default::Default::default(),
            format_version: core::default::Default::default(),
            padding_tolerance: core::default::Default::default(),
            row_index_stride: core::default::Default::default(),
            stripe_size_bytes: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `block_size_bytes` after provisioning.\n"]
    pub fn block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_size_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `bloom_filter_columns` after provisioning.\n"]
    pub fn bloom_filter_columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.bloom_filter_columns", self.base))
    }

    #[doc= "Get a reference to the value of field `bloom_filter_false_positive_probability` after provisioning.\n"]
    pub fn bloom_filter_false_positive_probability(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bloom_filter_false_positive_probability", self.base))
    }

    #[doc= "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc= "Get a reference to the value of field `dictionary_key_threshold` after provisioning.\n"]
    pub fn dictionary_key_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dictionary_key_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_padding` after provisioning.\n"]
    pub fn enable_padding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_padding", self.base))
    }

    #[doc= "Get a reference to the value of field `format_version` after provisioning.\n"]
    pub fn format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format_version", self.base))
    }

    #[doc= "Get a reference to the value of field `padding_tolerance` after provisioning.\n"]
    pub fn padding_tolerance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.padding_tolerance", self.base))
    }

    #[doc= "Get a reference to the value of field `row_index_stride` after provisioning.\n"]
    pub fn row_index_stride(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_index_stride", self.base))
    }

    #[doc= "Get a reference to the value of field `stripe_size_bytes` after provisioning.\n"]
    pub fn stripe_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.stripe_size_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_dictionary_compression: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_padding_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writer_version: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
    #[doc= "Set the field `block_size_bytes`.\n"]
    pub fn set_block_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_size_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_dictionary_compression`.\n"]
    pub fn set_enable_dictionary_compression(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_dictionary_compression = Some(v.into());
        self
    }

    #[doc= "Set the field `max_padding_bytes`.\n"]
    pub fn set_max_padding_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_padding_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `page_size_bytes`.\n"]
    pub fn set_page_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.page_size_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `writer_version`.\n"]
    pub fn set_writer_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.writer_version = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
            block_size_bytes: core::default::Default::default(),
            compression: core::default::Default::default(),
            enable_dictionary_compression: core::default::Default::default(),
            max_padding_bytes: core::default::Default::default(),
            page_size_bytes: core::default::Default::default(),
            writer_version: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `block_size_bytes` after provisioning.\n"]
    pub fn block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_size_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_dictionary_compression` after provisioning.\n"]
    pub fn enable_dictionary_compression(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_dictionary_compression", self.base))
    }

    #[doc= "Get a reference to the value of field `max_padding_bytes` after provisioning.\n"]
    pub fn max_padding_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_padding_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `page_size_bytes` after provisioning.\n"]
    pub fn page_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.page_size_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `writer_version` after provisioning.\n"]
    pub fn writer_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.writer_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElDynamic {
    orc_ser_de: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >,
    >,
    parquet_ser_de: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    orc_ser_de: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_ser_de: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    #[doc= "Set the field `orc_ser_de`.\n"]
    pub fn set_orc_ser_de(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.orc_ser_de = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.orc_ser_de = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `parquet_ser_de`.\n"]
    pub fn set_parquet_ser_de(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parquet_ser_de = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_ser_de = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
            orc_ser_de: core::default::Default::default(),
            parquet_ser_de: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `orc_ser_de` after provisioning.\n"]
    pub fn orc_ser_de(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.orc_ser_de", self.base))
    }

    #[doc= "Get a reference to the value of field `parquet_ser_de` after provisioning.\n"]
    pub fn parquet_ser_de(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parquet_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElDynamic {
    serializer: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    serializer: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    #[doc= "Set the field `serializer`.\n"]
    pub fn set_serializer(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.serializer = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.serializer = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
            serializer: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `serializer` after provisioning.\n"]
    pub fn serializer(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.serializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    #[doc= "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    #[doc= ""]
    pub database_name: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
    #[doc= ""]
    pub table_name: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            region: core::default::Default::default(),
            role_arn: self.role_arn,
            table_name: self.table_name,
            version_id: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc= "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElDynamic {
    input_format_configuration: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >,
    >,
    output_format_configuration: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >,
    >,
    schema_configuration: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_format_configuration: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format_configuration: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_configuration: Option<
        Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `input_format_configuration`.\n"]
    pub fn set_input_format_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_format_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_format_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `output_format_configuration`.\n"]
    pub fn set_output_format_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_format_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_format_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schema_configuration`.\n"]
    pub fn set_schema_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    type O =
        BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
            enabled: core::default::Default::default(),
            input_format_configuration: core::default::Default::default(),
            output_format_configuration: core::default::Default::default(),
            schema_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `input_format_configuration` after provisioning.\n"]
    pub fn input_format_configuration(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_format_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `output_format_configuration` after provisioning.\n"]
    pub fn output_format_configuration(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.output_format_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `schema_configuration` after provisioning.\n"]
    pub fn schema_configuration(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.schema_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_duration: Option<PrimField<f64>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_duration = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    type O =
        BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
            enabled: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc= ""]
    pub parameter_name: PrimField<String>,
    #[doc= ""]
    pub parameter_value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElDynamic {
    processors: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processors: Option<Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_output_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    #[doc= "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_format`.\n"]
    pub fn set_compression_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_format = Some(v.into());
        self
    }

    #[doc= "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_output_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    #[doc= ""]
    pub bucket_arn: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
            bucket_arn: self.bucket_arn,
            buffer_interval: core::default::Default::default(),
            buffer_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            cloudwatch_logging_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc= "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    data_format_conversion_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>,
    >,
    dynamic_partitioning_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>,
    >,
    processing_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>,
    >,
    s3_backup_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_output_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_format_conversion_configuration: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_partitioning_configuration: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<
        Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_configuration: Option<Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    #[doc= "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_format`.\n"]
    pub fn set_compression_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_format = Some(v.into());
        self
    }

    #[doc= "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_output_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_backup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `data_format_conversion_configuration`.\n"]
    pub fn set_data_format_conversion_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_format_conversion_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_format_conversion_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dynamic_partitioning_configuration`.\n"]
    pub fn set_dynamic_partitioning_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynamic_partitioning_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamic_partitioning_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `s3_backup_configuration`.\n"]
    pub fn set_s3_backup_configuration(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_backup_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_backup_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    #[doc= ""]
    pub bucket_arn: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
            bucket_arn: self.bucket_arn,
            buffer_interval: core::default::Default::default(),
            buffer_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            data_format_conversion_configuration: core::default::Default::default(),
            dynamic_partitioning_configuration: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_backup_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc= "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc= "Get a reference to the value of field `data_format_conversion_configuration` after provisioning.\n"]
    pub fn data_format_conversion_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_format_conversion_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `dynamic_partitioning_configuration` after provisioning.\n"]
    pub fn dynamic_partitioning_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dynamic_partitioning_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc= ""]
    pub parameter_name: PrimField<String>,
    #[doc= ""]
    pub parameter_value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElDynamic {
    processors: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processors: Option<
        Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElDynamic {
    common_attributes: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_attributes: Option<
        Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    #[doc= "Set the field `content_encoding`.\n"]
    pub fn set_content_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_encoding = Some(v.into());
        self
    }

    #[doc= "Set the field `common_attributes`.\n"]
    pub fn set_common_attributes(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.common_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.common_attributes = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
            content_encoding: core::default::Default::default(),
            common_attributes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content_encoding` after provisioning.\n"]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.base))
    }

    #[doc= "Get a reference to the value of field `common_attributes` after provisioning.\n"]
    pub fn common_attributes(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.common_attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>,
    >,
    request_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffering_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffering_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_mode: Option<PrimField<String>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<
        Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_configuration: Option<Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    #[doc= "Set the field `access_key`.\n"]
    pub fn set_access_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_key = Some(v.into());
        self
    }

    #[doc= "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffering_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffering_size = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_backup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `request_configuration`.\n"]
    pub fn set_request_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    #[doc= ""]
    pub url: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
            access_key: core::default::Default::default(),
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            name: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            s3_backup_mode: core::default::Default::default(),
            url: self.url,
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            request_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }

    #[doc= "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc= "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `request_configuration` after provisioning.\n"]
    pub fn request_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    kinesis_stream_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    #[doc= ""]
    pub kinesis_stream_arn: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
        KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
            kinesis_stream_arn: self.kinesis_stream_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
        KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kinesis_stream_arn` after provisioning.\n"]
    pub fn kinesis_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kinesis_stream_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc= ""]
    pub parameter_name: PrimField<String>,
    #[doc= ""]
    pub parameter_value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElDynamic {
    processors: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processors: Option<Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_output_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    #[doc= "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_format`.\n"]
    pub fn set_compression_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_format = Some(v.into());
        self
    }

    #[doc= "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_output_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    #[doc= ""]
    pub bucket_arn: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
            bucket_arn: self.bucket_arn,
            buffer_interval: core::default::Default::default(),
            buffer_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            cloudwatch_logging_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc= "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>,
    >,
    s3_backup_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    cluster_jdbcurl: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_table_columns: Option<PrimField<String>>,
    data_table_name: PrimField<String>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_duration: Option<PrimField<f64>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_mode: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<
        Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_configuration: Option<Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    #[doc= "Set the field `copy_options`.\n"]
    pub fn set_copy_options(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.copy_options = Some(v.into());
        self
    }

    #[doc= "Set the field `data_table_columns`.\n"]
    pub fn set_data_table_columns(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_table_columns = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_backup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `s3_backup_configuration`.\n"]
    pub fn set_s3_backup_configuration(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_backup_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_backup_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    #[doc= ""]
    pub cluster_jdbcurl: PrimField<String>,
    #[doc= ""]
    pub data_table_name: PrimField<String>,
    #[doc= ""]
    pub password: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
    #[doc= ""]
    pub username: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
            cluster_jdbcurl: self.cluster_jdbcurl,
            copy_options: core::default::Default::default(),
            data_table_columns: core::default::Default::default(),
            data_table_name: self.data_table_name,
            password: self.password,
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            username: self.username,
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_backup_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_jdbcurl` after provisioning.\n"]
    pub fn cluster_jdbcurl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_jdbcurl", self.base))
    }

    #[doc= "Get a reference to the value of field `copy_options` after provisioning.\n"]
    pub fn copy_options(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_options", self.base))
    }

    #[doc= "Get a reference to the value of field `data_table_columns` after provisioning.\n"]
    pub fn data_table_columns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_table_columns", self.base))
    }

    #[doc= "Get a reference to the value of field `data_table_name` after provisioning.\n"]
    pub fn data_table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_table_name", self.base))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc= "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamS3ConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_output_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<Vec<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamS3ConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    #[doc= "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_format`.\n"]
    pub fn set_compression_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_format = Some(v.into());
        self
    }

    #[doc= "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_output_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamS3ConfigurationEl {
    #[doc= ""]
    pub bucket_arn: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamS3ConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamS3ConfigurationEl {
        KinesisFirehoseDeliveryStreamS3ConfigurationEl {
            bucket_arn: self.bucket_arn,
            buffer_interval: core::default::Default::default(),
            buffer_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            cloudwatch_logging_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
        KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc= "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_type: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `key_arn`.\n"]
    pub fn set_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `key_type`.\n"]
    pub fn set_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_type = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamServerSideEncryptionEl {}

impl BuildKinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
        KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
            enabled: core::default::Default::default(),
            key_arn: core::default::Default::default(),
            key_type: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
        KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group_name = Some(v.into());
        self
    }

    #[doc= "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        BlockAssignable<
            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc= ""]
    pub parameter_name: PrimField<String>,
    #[doc= ""]
    pub parameter_value: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc= "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: Option<
        DynamicBlock<
            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<
        Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElDynamic {
    processors: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processors: Option<Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElDynamic {
    cloudwatch_logging_options: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: Option<
        DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hec_acknowledgment_timeout: Option<PrimField<f64>>,
    hec_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hec_endpoint_type: Option<PrimField<String>>,
    hec_token: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_backup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration: Option<Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    #[doc= "Set the field `hec_acknowledgment_timeout`.\n"]
    pub fn set_hec_acknowledgment_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hec_acknowledgment_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `hec_endpoint_type`.\n"]
    pub fn set_hec_endpoint_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hec_endpoint_type = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_backup_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v: impl Into<BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    type O = BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    #[doc= ""]
    pub hec_endpoint: PrimField<String>,
    #[doc= ""]
    pub hec_token: PrimField<String>,
}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
            hec_acknowledgment_timeout: core::default::Default::default(),
            hec_endpoint: self.hec_endpoint,
            hec_endpoint_type: core::default::Default::default(),
            hec_token: self.hec_token,
            retry_duration: core::default::Default::default(),
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hec_acknowledgment_timeout` after provisioning.\n"]
    pub fn hec_acknowledgment_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hec_acknowledgment_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `hec_endpoint` after provisioning.\n"]
    pub fn hec_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hec_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `hec_endpoint_type` after provisioning.\n"]
    pub fn hec_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hec_endpoint_type", self.base))
    }

    #[doc= "Get a reference to the value of field `hec_token` after provisioning.\n"]
    pub fn hec_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hec_token", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc= "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamDynamic {
    elasticsearch_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
    extended_s3_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
    http_endpoint_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
    kinesis_source_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
    redshift_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
    s3_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    server_side_encryption: Option<DynamicBlock<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
    splunk_configuration: Option<DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
}
