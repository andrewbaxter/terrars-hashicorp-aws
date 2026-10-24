use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KinesisFirehoseDeliveryStreamData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_s3_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_source_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    splunk_configuration: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamDynamic,
}

struct KinesisFirehoseDeliveryStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KinesisFirehoseDeliveryStreamData>,
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
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
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
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
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

    #[doc = "Set the field `kinesis_source_configuration`.\n"]
    pub fn set_kinesis_source_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
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

    #[doc = "Set the field `redshift_configuration`.\n"]
    pub fn set_redshift_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
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

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
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

    #[doc = "Set the field `splunk_configuration`.\n"]
    pub fn set_splunk_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
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
            data: std::cell::RefCell::new(KinesisFirehoseDeliveryStreamData {
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extended_s3_configuration` after provisioning.\n"]
    pub fn extended_s3_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_endpoint_configuration` after provisioning.\n"]
    pub fn http_endpoint_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kinesis_source_configuration` after provisioning.\n"]
    pub fn kinesis_source_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_source_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `splunk_configuration` after provisioning.\n"]
    pub fn splunk_configuration(&self) -> terrars::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splunk_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef {
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
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
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
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
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
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersEl,
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElDynamic {
    processors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef {
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
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElProcessorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    role_arn: terrars::PrimField<std::string::String>,
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef {
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
struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>,
    >,
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
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
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>>,
    dynamic: KinesisFirehoseDeliveryStreamElasticsearchConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationEl,
                        >,
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

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigEl>,
                    >,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamElasticsearchConfigurationEl {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
        KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamElasticsearchConfigurationElRef {
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamElasticsearchConfigurationElVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef {
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
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_formats: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestamp_formats` after provisioning.\n"]
    pub fn timestamp_formats(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_formats", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_insensitive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_to_json_key_mappings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    convert_dots_in_json_keys_to_underscores: std::option::Option<terrars::PrimField<bool>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef {
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
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElDynamic {
    hive_json_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >,
    >,
    open_x_json_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_json_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_x_json_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeEl,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeEl,
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hive_json_ser_de` after provisioning.\n"]
    pub fn hive_json_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElHiveJsonSerDeElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_json_ser_de", self.base))
    }

    #[doc = "Get a reference to the value of field `open_x_json_ser_de` after provisioning.\n"]
    pub fn open_x_json_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElOpenXJsonSerDeElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_x_json_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDynamic {
    deserializer: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deserializer: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerEl,
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deserializer` after provisioning.\n"]
    pub fn deserializer(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElDeserializerElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deserializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
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

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef {
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
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
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

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef {
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
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElDynamic {
    orc_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >,
    >,
    parquet_ser_de: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orc_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_ser_de: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeEl,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeEl,
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `orc_ser_de` after provisioning.\n"]
    pub fn orc_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElOrcSerDeElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orc_ser_de", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_ser_de` after provisioning.\n"]
    pub fn parquet_ser_de(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElParquetSerDeElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_ser_de", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElDynamic {
    serializer: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serializer: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerEl,
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `serializer` after provisioning.\n"]
    pub fn serializer(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElSerializerElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serializer", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
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

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef {
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
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElDynamic {
    input_format_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >,
    >,
    output_format_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >,
    >,
    schema_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationEl,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationEl,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationEl,
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef {
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
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElInputFormatConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_format_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format_configuration` after provisioning.\n"]
    pub fn output_format_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElOutputFormatConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_format_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_configuration` after provisioning.\n"]
    pub fn schema_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElSchemaConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
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
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef {
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

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
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
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersEl,
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElDynamic {
    processors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef {
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
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElProcessorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
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

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_size: std::option::Option<terrars::PrimField<f64>>,
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
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    #[doc = "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_size = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
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
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>>,
    data_format_conversion_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>,
    >,
    dynamic_partitioning_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>,
    >,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>>,
    s3_backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_size: std::option::Option<terrars::PrimField<f64>>,
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
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format_conversion_configuration: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_partitioning_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    #[doc = "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_size = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationEl,
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
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationEl,
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationEl,
                        >,
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationEl,
                        >,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
        KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
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

    #[doc = "Get a reference to the value of field `s3_backup_mode` after provisioning.\n"]
    pub fn s3_backup_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_backup_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `data_format_conversion_configuration` after provisioning.\n"]
    pub fn data_format_conversion_configuration(
        &self,
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDataFormatConversionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_format_conversion_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_partitioning_configuration` after provisioning.\n"]
    pub fn dynamic_partitioning_configuration(
        &self,
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElDynamicPartitioningConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_partitioning_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationElS3BackupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef {
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
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
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
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
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
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersEl,
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(
        self,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElDynamic {
    processors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef {
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
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElProcessorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef {
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
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElDynamic {
    common_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_attributes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef {
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
    ::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElCommonAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.common_attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    processing_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>,
    >,
    request_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
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
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationEl,
                        >,
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
                            KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationEl,
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
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl {
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
        KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRef {
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_configuration` after provisioning.\n"]
    pub fn request_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationElRequestConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    kinesis_stream_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl { }

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl {
    #[doc = ""]
    pub kinesis_stream_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
        KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamKinesisSourceConfigurationElRef {
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
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef {
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
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
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
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
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
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersEl,
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElDynamic {
    processors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<
        std::vec::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef {
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
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElProcessorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef {
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

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_size: std::option::Option<terrars::PrimField<f64>>,
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
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    #[doc = "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_size = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
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
    ) -> terrars
    ::ListRef<
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElCloudwatchLoggingOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>>,
    s3_backup_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    cluster_jdbcurl: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_table_columns: std::option::Option<terrars::PrimField<std::string::String>>,
    data_table_name: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamRedshiftConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationEl,
                        >,
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
                        ::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationEl>,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamRedshiftConfigurationEl {
    #[doc = ""]
    pub cluster_jdbcurl: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub data_table_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
        KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamRedshiftConfigurationElRef {
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_backup_configuration` after provisioning.\n"]
    pub fn s3_backup_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamRedshiftConfigurationElS3BackupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_backup_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef {
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

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamS3ConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_size: std::option::Option<terrars::PrimField<f64>>,
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
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamS3ConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    #[doc = "Set the field `buffer_interval`.\n"]
    pub fn set_buffer_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_size`.\n"]
    pub fn set_buffer_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_size = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsEl>,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamS3ConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamS3ConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamS3ConfigurationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
        KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamS3ConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_interval` after provisioning.\n"]
    pub fn buffer_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamS3ConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamServerSideEncryptionEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
        KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamServerSideEncryptionElRef {
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
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef {
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
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    parameter_name: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl { }

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef {
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
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElDynamic {
    parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl>,
    >,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
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
                            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersEl,
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

impl terrars
::ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    type O =
        terrars
        ::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
    pub fn build(self) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> terrars
    ::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElDynamic {
    processors: std
    ::option
    ::Option<
        terrars::DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processors: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl>>,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
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
                            KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsEl,
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

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef {
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElProcessorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processors", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamSplunkConfigurationElDynamic {
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hec_acknowledgment_timeout: std::option::Option<terrars::PrimField<f64>>,
    hec_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hec_endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
    hec_token: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_backup_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std
    ::option
    ::Option<std::vec::Vec<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>>,
    dynamic: KinesisFirehoseDeliveryStreamSplunkConfigurationElDynamic,
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
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
                        ::BlockAssignable<
                            KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsEl,
                        >,
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
                        ::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationEl>,
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
}

impl terrars::ToListMappable for KinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    type O = terrars::BlockAssignable<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisFirehoseDeliveryStreamSplunkConfigurationEl {
    #[doc = ""]
    pub hec_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub hec_token: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
        KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisFirehoseDeliveryStreamSplunkConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> terrars::ListRef<KinesisFirehoseDeliveryStreamSplunkConfigurationElProcessingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisFirehoseDeliveryStreamDynamic {
    elasticsearch_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamElasticsearchConfigurationEl>>,
    extended_s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamExtendedS3ConfigurationEl>>,
    http_endpoint_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamHttpEndpointConfigurationEl>>,
    kinesis_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamKinesisSourceConfigurationEl>>,
    redshift_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamRedshiftConfigurationEl>>,
    s3_configuration: std::option::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamS3ConfigurationEl>>,
    server_side_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamServerSideEncryptionEl>>,
    splunk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KinesisFirehoseDeliveryStreamSplunkConfigurationEl>>,
}
