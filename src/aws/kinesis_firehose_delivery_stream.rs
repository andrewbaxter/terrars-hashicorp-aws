#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KinesisFirehoseDeliveryStreamData_ {
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
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_ElasticsearchConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_s3_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_ExtendedS3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_IcebergConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_source_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_KinesisSourceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msk_source_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_MskSourceConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearch_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearchserverless_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_RedshiftConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_ServerSideEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SnowflakeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    splunk_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KinesisFirehoseDeliveryStream_Timeouts>,
    dynamic: KinesisFirehoseDeliveryStreamDynamic,
}

struct KinesisFirehoseDeliveryStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KinesisFirehoseDeliveryStreamData_>,
}

#[derive(Clone)]
pub struct KinesisFirehoseDeliveryStream(std::rc::Rc<KinesisFirehoseDeliveryStream_>);

impl KinesisFirehoseDeliveryStream {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_id`.\n"]
    pub fn set_destination_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().destination_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elasticsearch_configuration`.\n"]
    pub fn set_elasticsearch_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ElasticsearchConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extended_s3_configuration`.\n"]
    pub fn set_extended_s3_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ExtendedS3Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extended_s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extended_s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_endpoint_configuration`.\n"]
    pub fn set_http_endpoint_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_endpoint_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_endpoint_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_IcebergConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iceberg_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iceberg_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_source_configuration`.\n"]
    pub fn set_kinesis_source_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_KinesisSourceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `msk_source_configuration`.\n"]
    pub fn set_msk_source_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_MskSourceConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().msk_source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.msk_source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearch_configuration`.\n"]
    pub fn set_opensearch_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().opensearch_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.opensearch_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearchserverless_configuration`.\n"]
    pub fn set_opensearchserverless_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().opensearchserverless_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.opensearchserverless_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_configuration`.\n"]
    pub fn set_redshift_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_RedshiftConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ServerSideEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake_configuration`.\n"]
    pub fn set_snowflake_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SnowflakeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snowflake_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snowflake_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `splunk_configuration`.\n"]
    pub fn set_splunk_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().splunk_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.splunk_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KinesisFirehoseDeliveryStream_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_id` after provisioning.\n"]
    pub fn destination_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_configuration` after provisioning.\n"]
    pub fn elasticsearch_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_IcebergConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `msk_source_configuration` after provisioning.\n"]
    pub fn msk_source_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_MskSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msk_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearch_configuration` after provisioning.\n"]
    pub fn opensearch_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearchserverless_configuration` after provisioning.\n"]
    pub fn opensearchserverless_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.opensearchserverless_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_RedshiftConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snowflake_configuration` after provisioning.\n"]
    pub fn snowflake_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KinesisFirehoseDeliveryStream_TimeoutsRef {
        KinesisFirehoseDeliveryStream_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for KinesisFirehoseDeliveryStream {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KinesisFirehoseDeliveryStream { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream {
    type O = terrars::ListRef<KinesisFirehoseDeliveryStreamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KinesisFirehoseDeliveryStream_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kinesis_firehose_delivery_stream".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisFirehoseDeliveryStream {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream {
    pub fn build(self, stack: &mut terrars::Stack) -> KinesisFirehoseDeliveryStream {
        let out = KinesisFirehoseDeliveryStream(std::rc::Rc::new(KinesisFirehoseDeliveryStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KinesisFirehoseDeliveryStreamData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                arn: core::default::Default::default(),
                destination: self.destination,
                destination_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_id: core::default::Default::default(),
                elasticsearch_configuration: core::default::Default::default(),
                extended_s3_configuration: core::default::Default::default(),
                http_endpoint_configuration: core::default::Default::default(),
                iceberg_configuration: core::default::Default::default(),
                kinesis_source_configuration: core::default::Default::default(),
                msk_source_configuration: core::default::Default::default(),
                opensearch_configuration: core::default::Default::default(),
                opensearchserverless_configuration: core::default::Default::default(),
                redshift_configuration: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                snowflake_configuration: core::default::Default::default(),
                splunk_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KinesisFirehoseDeliveryStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KinesisFirehoseDeliveryStreamRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_id` after provisioning.\n"]
    pub fn destination_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_configuration` after provisioning.\n"]
    pub fn elasticsearch_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_IcebergConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `msk_source_configuration` after provisioning.\n"]
    pub fn msk_source_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_MskSourceConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msk_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearch_configuration` after provisioning.\n"]
    pub fn opensearch_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `opensearchserverless_configuration` after provisioning.\n"]
    pub fn opensearchserverless_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.opensearchserverless_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_RedshiftConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snowflake_configuration` after provisioning.\n"]
    pub fn snowflake_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KinesisFirehoseDeliveryStream_TimeoutsRef {
        KinesisFirehoseDeliveryStream_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamDynamic {
    elasticsearch_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_ElasticsearchConfiguration>>,
    extended_s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_ExtendedS3Configuration>>,
    http_endpoint_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration>>,
    iceberg_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_IcebergConfiguration>>,
    kinesis_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_KinesisSourceConfiguration>>,
    msk_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_MskSourceConfiguration>>,
    opensearch_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchConfiguration>>,
    opensearchserverless_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration>>,
    redshift_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_RedshiftConfiguration>>,
    server_side_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_ServerSideEncryption>>,
    snowflake_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SnowflakeConfiguration>>,
    splunk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_stream_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions {
            enabled: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_ParametersRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters>,
    >,
    dynamic: KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsDynamic,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors_Parameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationDynamic {
    processors: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors>>,
    dynamic: KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `processors`.\n"]
    pub fn set_processors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_Processors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration {
            enabled: core::default::Default::default(),
            processors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `processors` after provisioning.\n"]
    pub fn processors(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration_ProcessorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    dynamic: KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_format`.\n"]
    pub fn set_compression_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compression_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_output_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration {
            bucket_arn: self.bucket_arn,
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            cloudwatch_logging_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc = "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
    role_arn: terrars::PrimField<std::string::String>,
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
        KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig {
            role_arn: self.role_arn,
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef {
        KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ElasticsearchConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    dynamic: KinesisFirehoseDeliveryStream_ElasticsearchConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_endpoint`.\n"]
    pub fn set_cluster_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_arn`.\n"]
    pub fn set_domain_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_rotation_period`.\n"]
    pub fn set_index_rotation_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.index_rotation_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_name`.\n"]
    pub fn set_type_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ElasticsearchConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
        KinesisFirehoseDeliveryStream_ElasticsearchConfiguration {
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
            s3_configuration: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef {
        KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ElasticsearchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_endpoint` after provisioning.\n"]
    pub fn cluster_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_arn` after provisioning.\n"]
    pub fn domain_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `index_rotation_period` after provisioning.\n"]
    pub fn index_rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_rotation_period", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_formats: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
    #[doc = "Set the field `timestamp_formats`.\n"]
    pub fn set_timestamp_formats(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.timestamp_formats = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe {
            timestamp_formats: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestamp_formats` after provisioning.\n"]
    pub fn timestamp_formats(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_formats", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_insensitive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_to_json_key_mappings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_dots_in_json_keys_to_underscores: std::option::Option<terrars::PrimField<bool>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
    #[doc = "Set the field `case_insensitive`.\n"]
    pub fn set_case_insensitive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.case_insensitive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_to_json_key_mappings`.\n"]
    pub fn set_column_to_json_key_mappings(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_to_json_key_mappings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `convert_dots_in_json_keys_to_underscores`.\n"]
    pub fn set_convert_dots_in_json_keys_to_underscores(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.convert_dots_in_json_keys_to_underscores = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe {
            case_insensitive: core::default::Default::default(),
            column_to_json_key_mappings: core::default::Default::default(),
            convert_dots_in_json_keys_to_underscores: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_insensitive` after provisioning.\n"]
    pub fn case_insensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_insensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `column_to_json_key_mappings` after provisioning.\n"]
    pub fn column_to_json_key_mappings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.column_to_json_key_mappings", self.base))
    }

    #[doc = "Get a reference to the value of field `convert_dots_in_json_keys_to_underscores` after provisioning.\n"]
    pub fn convert_dots_in_json_keys_to_underscores(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.convert_dots_in_json_keys_to_underscores", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerDynamic {
    hive_json_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe,
        >,
    >,
    open_x_json_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_json_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_x_json_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
    #[doc = "Set the field `hive_json_ser_de`.\n"]
    pub fn set_hive_json_ser_de(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hive_json_ser_de = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hive_json_ser_de = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_x_json_ser_de`.\n"]
    pub fn set_open_x_json_ser_de(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_x_json_ser_de = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_x_json_ser_de = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer {
            hive_json_ser_de: core::default::Default::default(),
            open_x_json_ser_de: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hive_json_ser_de` after provisioning.\n"]
    pub fn hive_json_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_HiveJsonSerDeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_json_ser_de", self.base))
    }

    #[doc = "Get a reference to the value of field `open_x_json_ser_de` after provisioning.\n"]
    pub fn open_x_json_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer_OpenXJsonSerDeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_x_json_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationDynamic {
    deserializer: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deserializer: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
    #[doc = "Set the field `deserializer`.\n"]
    pub fn set_deserializer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_Deserializer,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deserializer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deserializer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration {
            deserializer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deserializer` after provisioning.\n"]
    pub fn deserializer(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration_DeserializerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deserializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bloom_filter_columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bloom_filter_false_positive_probability: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary_key_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_padding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    padding_tolerance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_index_stride: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stripe_size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
    #[doc = "Set the field `block_size_bytes`.\n"]
    pub fn set_block_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bloom_filter_columns`.\n"]
    pub fn set_bloom_filter_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bloom_filter_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bloom_filter_false_positive_probability`.\n"]
    pub fn set_bloom_filter_false_positive_probability(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.bloom_filter_false_positive_probability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dictionary_key_threshold`.\n"]
    pub fn set_dictionary_key_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dictionary_key_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_padding`.\n"]
    pub fn set_enable_padding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_padding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format_version`.\n"]
    pub fn set_format_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `padding_tolerance`.\n"]
    pub fn set_padding_tolerance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.padding_tolerance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_index_stride`.\n"]
    pub fn set_row_index_stride(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_index_stride = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stripe_size_bytes`.\n"]
    pub fn set_stripe_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.stripe_size_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe {
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

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_size_bytes` after provisioning.\n"]
    pub fn block_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_size_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `bloom_filter_columns` after provisioning.\n"]
    pub fn bloom_filter_columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bloom_filter_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `bloom_filter_false_positive_probability` after provisioning.\n"]
    pub fn bloom_filter_false_positive_probability(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bloom_filter_false_positive_probability", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary_key_threshold` after provisioning.\n"]
    pub fn dictionary_key_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dictionary_key_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_padding` after provisioning.\n"]
    pub fn enable_padding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_padding", self.base))
    }

    #[doc = "Get a reference to the value of field `format_version` after provisioning.\n"]
    pub fn format_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format_version", self.base))
    }

    #[doc = "Get a reference to the value of field `padding_tolerance` after provisioning.\n"]
    pub fn padding_tolerance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.padding_tolerance", self.base))
    }

    #[doc = "Get a reference to the value of field `row_index_stride` after provisioning.\n"]
    pub fn row_index_stride(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_index_stride", self.base))
    }

    #[doc = "Get a reference to the value of field `stripe_size_bytes` after provisioning.\n"]
    pub fn stripe_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stripe_size_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dictionary_compression: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_padding_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    writer_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
    #[doc = "Set the field `block_size_bytes`.\n"]
    pub fn set_block_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dictionary_compression`.\n"]
    pub fn set_enable_dictionary_compression(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_dictionary_compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_padding_bytes`.\n"]
    pub fn set_max_padding_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_padding_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `page_size_bytes`.\n"]
    pub fn set_page_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.page_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `writer_version`.\n"]
    pub fn set_writer_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.writer_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe {
            block_size_bytes: core::default::Default::default(),
            compression: core::default::Default::default(),
            enable_dictionary_compression: core::default::Default::default(),
            max_padding_bytes: core::default::Default::default(),
            page_size_bytes: core::default::Default::default(),
            writer_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_size_bytes` after provisioning.\n"]
    pub fn block_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_size_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_dictionary_compression` after provisioning.\n"]
    pub fn enable_dictionary_compression(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dictionary_compression", self.base))
    }

    #[doc = "Get a reference to the value of field `max_padding_bytes` after provisioning.\n"]
    pub fn max_padding_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_padding_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `page_size_bytes` after provisioning.\n"]
    pub fn page_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `writer_version` after provisioning.\n"]
    pub fn writer_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.writer_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerDynamic {
    orc_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe,
        >,
    >,
    parquet_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orc_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
    #[doc = "Set the field `orc_ser_de`.\n"]
    pub fn set_orc_ser_de(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.orc_ser_de = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.orc_ser_de = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parquet_ser_de`.\n"]
    pub fn set_parquet_ser_de(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDe,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parquet_ser_de = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_ser_de = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer {
            orc_ser_de: core::default::Default::default(),
            parquet_ser_de: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `orc_ser_de` after provisioning.\n"]
    pub fn orc_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_OrcSerDeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orc_ser_de", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_ser_de` after provisioning.\n"]
    pub fn parquet_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer_ParquetSerDeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationDynamic {
    serializer: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serializer: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
    #[doc = "Set the field `serializer`.\n"]
    pub fn set_serializer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_Serializer,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serializer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serializer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration {
            serializer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `serializer` after provisioning.\n"]
    pub fn serializer(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration_SerializerRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            region: core::default::Default::default(),
            role_arn: self.role_arn,
            table_name: self.table_name,
            version_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationDynamic {
    input_format_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration,
        >,
    >,
    output_format_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration,
        >,
    >,
    schema_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_format_configuration`.\n"]
    pub fn set_input_format_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_format_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_format_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_format_configuration`.\n"]
    pub fn set_output_format_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_format_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_format_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_configuration`.\n"]
    pub fn set_schema_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration {
            enabled: core::default::Default::default(),
            input_format_configuration: core::default::Default::default(),
            output_format_configuration: core::default::Default::default(),
            schema_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `input_format_configuration` after provisioning.\n"]
    pub fn input_format_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_InputFormatConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_format_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format_configuration` after provisioning.\n"]
    pub fn output_format_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_OutputFormatConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_format_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_configuration` after provisioning.\n"]
    pub fn schema_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration_SchemaConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration {
            enabled: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    dynamic: KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_format`.\n"]
    pub fn set_compression_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compression_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_output_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
        KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration {
            bucket_arn: self.bucket_arn,
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            cloudwatch_logging_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef {
        KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc = "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    data_format_conversion_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration>,
    >,
    dynamic_partitioning_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration>,
    >,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3Configuration {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_extension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format_conversion_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_partitioning_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_ExtendedS3Configuration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_format`.\n"]
    pub fn set_compression_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compression_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_time_zone`.\n"]
    pub fn set_custom_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_output_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_extension`.\n"]
    pub fn set_file_extension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_extension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_format_conversion_configuration`.\n"]
    pub fn set_data_format_conversion_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_format_conversion_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_format_conversion_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamic_partitioning_configuration`.\n"]
    pub fn set_dynamic_partitioning_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynamic_partitioning_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamic_partitioning_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_backup_configuration`.\n"]
    pub fn set_s3_backup_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_backup_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_backup_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_ExtendedS3Configuration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ExtendedS3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_ExtendedS3Configuration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_ExtendedS3Configuration {
        KinesisFirehoseDeliveryStream_ExtendedS3Configuration {
            bucket_arn: self.bucket_arn,
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            compression_format: core::default::Default::default(),
            custom_time_zone: core::default::Default::default(),
            error_output_prefix: core::default::Default::default(),
            file_extension: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            data_format_conversion_configuration: core::default::Default::default(),
            dynamic_partitioning_configuration: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_backup_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef {
        KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ExtendedS3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_format` after provisioning.\n"]
    pub fn compression_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_format", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_time_zone` after provisioning.\n"]
    pub fn custom_time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `file_extension` after provisioning.\n"]
    pub fn file_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_extension", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `data_format_conversion_configuration` after provisioning.\n"]
    pub fn data_format_conversion_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DataFormatConversionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_format_conversion_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_partitioning_configuration` after provisioning.\n"]
    pub fn dynamic_partitioning_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_ExtendedS3Configuration_DynamicPartitioningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_partitioning_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
        KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef {
        KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationDynamic {
    common_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_attributes: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes>,
    >,
    dynamic: KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
    #[doc = "Set the field `content_encoding`.\n"]
    pub fn set_content_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `common_attributes`.\n"]
    pub fn set_common_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.common_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.common_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
        KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration {
            content_encoding: core::default::Default::default(),
            common_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef {
        KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_encoding` after provisioning.\n"]
    pub fn content_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `common_attributes` after provisioning.\n"]
    pub fn common_attributes(
        &self,
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration_CommonAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration {
            enabled: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef {
        KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_HttpEndpointConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    request_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    secrets_manager_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_HttpEndpointConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
    #[doc = "Set the field `access_key`.\n"]
    pub fn set_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_configuration`.\n"]
    pub fn set_request_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets_manager_configuration`.\n"]
    pub fn set_secrets_manager_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_manager_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_manager_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
        KinesisFirehoseDeliveryStream_HttpEndpointConfiguration {
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
            s3_configuration: core::default::Default::default(),
            secrets_manager_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef {
        KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_HttpEndpointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_configuration` after provisioning.\n"]
    pub fn request_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_HttpEndpointConfiguration_RequestConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_configuration` after provisioning.\n"]
    pub fn secrets_manager_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_error_output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
    #[doc = "Set the field `s3_error_output_prefix`.\n"]
    pub fn set_s3_error_output_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_error_output_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unique_keys`.\n"]
    pub fn set_unique_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.unique_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
        KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration {
            database_name: self.database_name,
            s3_error_output_prefix: core::default::Default::default(),
            table_name: self.table_name,
            unique_keys: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef {
        KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_error_output_prefix` after provisioning.\n"]
    pub fn s3_error_output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_error_output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_keys` after provisioning.\n"]
    pub fn unique_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unique_keys", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_IcebergConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    destination_table_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_IcebergConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    catalog_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    dynamic: KinesisFirehoseDeliveryStream_IcebergConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_IcebergConfiguration {
    #[doc = "Set the field `append_only`.\n"]
    pub fn set_append_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.append_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_table_configuration`.\n"]
    pub fn set_destination_table_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_table_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_IcebergConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_IcebergConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_IcebergConfiguration {
    #[doc = ""]
    pub catalog_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_IcebergConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_IcebergConfiguration {
        KinesisFirehoseDeliveryStream_IcebergConfiguration {
            append_only: core::default::Default::default(),
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            catalog_arn: self.catalog_arn,
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            destination_table_configuration: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_IcebergConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_IcebergConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_IcebergConfigurationRef {
        KinesisFirehoseDeliveryStream_IcebergConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_IcebergConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_only` after provisioning.\n"]
    pub fn append_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_only", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog_arn` after provisioning.\n"]
    pub fn catalog_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table_configuration` after provisioning.\n"]
    pub fn destination_table_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_IcebergConfiguration_DestinationTableConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_table_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
    kinesis_stream_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_KinesisSourceConfiguration { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_KinesisSourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
    #[doc = ""]
    pub kinesis_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
        KinesisFirehoseDeliveryStream_KinesisSourceConfiguration {
            kinesis_stream_arn: self.kinesis_stream_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef {
        KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_KinesisSourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_arn` after provisioning.\n"]
    pub fn kinesis_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kinesis_stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
    connectivity: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
    #[doc = ""]
    pub connectivity: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
        KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration {
            connectivity: self.connectivity,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef {
        KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connectivity` after provisioning.\n"]
    pub fn connectivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connectivity", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_MskSourceConfigurationDynamic {
    authentication_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_MskSourceConfiguration {
    msk_cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_from_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    topic_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_MskSourceConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_MskSourceConfiguration {
    #[doc = "Set the field `read_from_timestamp`.\n"]
    pub fn set_read_from_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.read_from_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_MskSourceConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_MskSourceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_MskSourceConfiguration {
    #[doc = ""]
    pub msk_cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_MskSourceConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_MskSourceConfiguration {
        KinesisFirehoseDeliveryStream_MskSourceConfiguration {
            msk_cluster_arn: self.msk_cluster_arn,
            read_from_timestamp: core::default::Default::default(),
            topic_name: self.topic_name,
            authentication_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_MskSourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_MskSourceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_MskSourceConfigurationRef {
        KinesisFirehoseDeliveryStream_MskSourceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_MskSourceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `msk_cluster_arn` after provisioning.\n"]
    pub fn msk_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.msk_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `read_from_timestamp` after provisioning.\n"]
    pub fn read_from_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_from_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_name` after provisioning.\n"]
    pub fn topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_MskSourceConfiguration_AuthenticationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
    default_document_id_format: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
    #[doc = ""]
    pub default_document_id_format: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
        KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions {
            default_document_id_format: self.default_document_id_format,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef {
        KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_document_id_format` after provisioning.\n"]
    pub fn default_document_id_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_document_id_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_OpensearchConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    document_id_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_id_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    dynamic: KinesisFirehoseDeliveryStream_OpensearchConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_OpensearchConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_endpoint`.\n"]
    pub fn set_cluster_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_arn`.\n"]
    pub fn set_domain_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_rotation_period`.\n"]
    pub fn set_index_rotation_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.index_rotation_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_name`.\n"]
    pub fn set_type_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_id_options`.\n"]
    pub fn set_document_id_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_id_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_id_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_OpensearchConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_OpensearchConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_OpensearchConfiguration {
        KinesisFirehoseDeliveryStream_OpensearchConfiguration {
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
            document_id_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_OpensearchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_OpensearchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_OpensearchConfigurationRef {
        KinesisFirehoseDeliveryStream_OpensearchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_OpensearchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_endpoint` after provisioning.\n"]
    pub fn cluster_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_arn` after provisioning.\n"]
    pub fn domain_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `index_rotation_period` after provisioning.\n"]
    pub fn index_rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_rotation_period", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `document_id_options` after provisioning.\n"]
    pub fn document_id_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfiguration_DocumentIdOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_id_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    collection_endpoint: terrars::PrimField<std::string::String>,
    index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    dynamic: KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
    #[doc = ""]
    pub collection_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
        KinesisFirehoseDeliveryStream_OpensearchserverlessConfiguration {
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            collection_endpoint: self.collection_endpoint,
            index_name: self.index_name,
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef {
        KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_OpensearchserverlessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_endpoint` after provisioning.\n"]
    pub fn collection_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStream_OpensearchConfiguration_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_RedshiftConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    secrets_manager_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_RedshiftConfiguration {
    cluster_jdbcurl: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_table_columns: std::option::Option<terrars::PrimField<std::string::String>>,
    data_table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_RedshiftConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_RedshiftConfiguration {
    #[doc = "Set the field `copy_options`.\n"]
    pub fn set_copy_options(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.copy_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_table_columns`.\n"]
    pub fn set_data_table_columns(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_table_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_backup_configuration`.\n"]
    pub fn set_s3_backup_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_backup_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_backup_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets_manager_configuration`.\n"]
    pub fn set_secrets_manager_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_manager_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_manager_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_RedshiftConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_RedshiftConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_RedshiftConfiguration {
    #[doc = ""]
    pub cluster_jdbcurl: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub data_table_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_RedshiftConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_RedshiftConfiguration {
        KinesisFirehoseDeliveryStream_RedshiftConfiguration {
            cluster_jdbcurl: self.cluster_jdbcurl,
            copy_options: core::default::Default::default(),
            data_table_columns: core::default::Default::default(),
            data_table_name: self.data_table_name,
            password: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            username: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_backup_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            secrets_manager_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_RedshiftConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_RedshiftConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_RedshiftConfigurationRef {
        KinesisFirehoseDeliveryStream_RedshiftConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_RedshiftConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_jdbcurl` after provisioning.\n"]
    pub fn cluster_jdbcurl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_jdbcurl", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_options` after provisioning.\n"]
    pub fn copy_options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_options", self.base))
    }

    #[doc = "Get a reference to the value of field `data_table_columns` after provisioning.\n"]
    pub fn data_table_columns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `data_table_name` after provisioning.\n"]
    pub fn data_table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_RedshiftConfiguration_S3BackupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_configuration` after provisioning.\n"]
    pub fn secrets_manager_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_ServerSideEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_ServerSideEncryption {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_arn`.\n"]
    pub fn set_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_type`.\n"]
    pub fn set_key_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_ServerSideEncryption {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_ServerSideEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_ServerSideEncryption {}

impl BuildKinesisFirehoseDeliveryStream_ServerSideEncryption {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_ServerSideEncryption {
        KinesisFirehoseDeliveryStream_ServerSideEncryption {
            enabled: core::default::Default::default(),
            key_arn: core::default::Default::default(),
            key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_ServerSideEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_ServerSideEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_ServerSideEncryptionRef {
        KinesisFirehoseDeliveryStream_ServerSideEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_ServerSideEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake_role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snowflake_role`.\n"]
    pub fn set_snowflake_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snowflake_role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {}

impl BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
        KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration {
            enabled: core::default::Default::default(),
            snowflake_role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef {
        KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake_role` after provisioning.\n"]
    pub fn snowflake_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snowflake_role", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
    private_link_vpce_id: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
    #[doc = ""]
    pub private_link_vpce_id: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
        KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration {
            private_link_vpce_id: self.private_link_vpce_id,
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef {
        KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_link_vpce_id` after provisioning.\n"]
    pub fn private_link_vpce_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_link_vpce_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_SnowflakeConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    secrets_manager_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
    snowflake_role_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration>>,
    snowflake_vpc_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfiguration {
    account_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_loading_option: std::option::Option<terrars::PrimField<std::string::String>>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    schema: terrars::PrimField<std::string::String>,
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake_role_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake_vpc_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_SnowflakeConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_column_name`.\n"]
    pub fn set_content_column_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_loading_option`.\n"]
    pub fn set_data_loading_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_loading_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_passphrase`.\n"]
    pub fn set_key_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_passphrase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_column_name`.\n"]
    pub fn set_metadata_column_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metadata_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets_manager_configuration`.\n"]
    pub fn set_secrets_manager_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_manager_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_manager_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake_role_configuration`.\n"]
    pub fn set_snowflake_role_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake_role_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake_role_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake_vpc_configuration`.\n"]
    pub fn set_snowflake_vpc_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake_vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake_vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SnowflakeConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SnowflakeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration {
    #[doc = ""]
    pub account_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub schema: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SnowflakeConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SnowflakeConfiguration {
        KinesisFirehoseDeliveryStream_SnowflakeConfiguration {
            account_url: self.account_url,
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            content_column_name: core::default::Default::default(),
            data_loading_option: core::default::Default::default(),
            database: self.database,
            key_passphrase: core::default::Default::default(),
            metadata_column_name: core::default::Default::default(),
            private_key: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_backup_mode: core::default::Default::default(),
            schema: self.schema,
            table: self.table,
            user: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            secrets_manager_configuration: core::default::Default::default(),
            snowflake_role_configuration: core::default::Default::default(),
            snowflake_vpc_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef {
        KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SnowflakeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_url` after provisioning.\n"]
    pub fn account_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_url", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `content_column_name` after provisioning.\n"]
    pub fn content_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `data_loading_option` after provisioning.\n"]
    pub fn data_loading_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_loading_option", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `key_passphrase` after provisioning.\n"]
    pub fn key_passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_passphrase", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_column_name` after provisioning.\n"]
    pub fn metadata_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_configuration` after provisioning.\n"]
    pub fn secrets_manager_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake_role_configuration` after provisioning.\n"]
    pub fn snowflake_role_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeRoleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake_role_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake_vpc_configuration` after provisioning.\n"]
    pub fn snowflake_vpc_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SnowflakeConfiguration_SnowflakeVpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake_vpc_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStream_SplunkConfigurationDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    secrets_manager_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffering_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hec_acknowledgment_timeout: std::option::Option<terrars::PrimField<f64>>,
    hec_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hec_endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hec_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration>>,
    dynamic: KinesisFirehoseDeliveryStream_SplunkConfigurationDynamic,
}

impl KinesisFirehoseDeliveryStream_SplunkConfiguration {
    #[doc = "Set the field `buffering_interval`.\n"]
    pub fn set_buffering_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffering_size`.\n"]
    pub fn set_buffering_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffering_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hec_acknowledgment_timeout`.\n"]
    pub fn set_hec_acknowledgment_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hec_acknowledgment_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hec_endpoint_type`.\n"]
    pub fn set_hec_endpoint_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hec_endpoint_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hec_token`.\n"]
    pub fn set_hec_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hec_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_duration`.\n"]
    pub fn set_retry_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_backup_mode`.\n"]
    pub fn set_s3_backup_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_backup_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets_manager_configuration`.\n"]
    pub fn set_secrets_manager_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_manager_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_manager_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_SplunkConfiguration {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_SplunkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_SplunkConfiguration {
    #[doc = ""]
    pub hec_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStream_SplunkConfiguration {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_SplunkConfiguration {
        KinesisFirehoseDeliveryStream_SplunkConfiguration {
            buffering_interval: core::default::Default::default(),
            buffering_size: core::default::Default::default(),
            hec_acknowledgment_timeout: core::default::Default::default(),
            hec_endpoint: self.hec_endpoint,
            hec_endpoint_type: core::default::Default::default(),
            hec_token: core::default::Default::default(),
            retry_duration: core::default::Default::default(),
            s3_backup_mode: core::default::Default::default(),
            cloudwatch_logging_options: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            secrets_manager_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_SplunkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_SplunkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStream_SplunkConfigurationRef {
        KinesisFirehoseDeliveryStream_SplunkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_SplunkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buffering_interval` after provisioning.\n"]
    pub fn buffering_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffering_size` after provisioning.\n"]
    pub fn buffering_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffering_size", self.base))
    }

    #[doc = "Get a reference to the value of field `hec_acknowledgment_timeout` after provisioning.\n"]
    pub fn hec_acknowledgment_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hec_acknowledgment_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `hec_endpoint` after provisioning.\n"]
    pub fn hec_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hec_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hec_endpoint_type` after provisioning.\n"]
    pub fn hec_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hec_endpoint_type", self.base))
    }

    #[doc = "Get a reference to the value of field `hec_token` after provisioning.\n"]
    pub fn hec_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hec_token", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_duration` after provisioning.\n"]
    pub fn retry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_configuration` after provisioning.\n"]
    pub fn secrets_manager_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStream_SplunkConfiguration_SecretsManagerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStream_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStream_Timeouts {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStream_Timeouts {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStream_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStream_Timeouts {}

impl BuildKinesisFirehoseDeliveryStream_Timeouts {
    pub fn build(self) -> KinesisFirehoseDeliveryStream_Timeouts {
        KinesisFirehoseDeliveryStream_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KinesisFirehoseDeliveryStream_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStream_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KinesisFirehoseDeliveryStream_TimeoutsRef {
        KinesisFirehoseDeliveryStream_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStream_TimeoutsRef {
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
