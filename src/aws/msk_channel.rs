#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskChannelData_ {
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
    channel_name: terrars::PrimField<std::string::String>,
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<MskChannel_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_destination: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_info: std::option::Option<std::vec::Vec<MskChannel_LoggingInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std::option::Option<std::vec::Vec<MskChannel_S3Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskChannel_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_configuration: std::option::Option<std::vec::Vec<MskChannel_TopicConfiguration>>,
    dynamic: MskChannelDynamic,
}

struct MskChannel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskChannelData_>,
}

#[derive(Clone)]
pub struct MskChannel(std::rc::Rc<MskChannel_>);

impl MskChannel {
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_EncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iceberg_destination`.\n"]
    pub fn set_iceberg_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_IcebergDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iceberg_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iceberg_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_info`.\n"]
    pub fn set_logging_info(self, v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_LoggingInfo>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_S3Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskChannel_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_configuration`.\n"]
    pub fn set_topic_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_TopicConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topic_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topic_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<MskChannel_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_destination` after provisioning.\n"]
    pub fn iceberg_destination(&self) -> terrars::ListRef<MskChannel_IcebergDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskChannel_LoggingInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<MskChannel_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskChannel_TimeoutsRef {
        MskChannel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_configuration` after provisioning.\n"]
    pub fn topic_configuration(&self) -> terrars::ListRef<MskChannel_TopicConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for MskChannel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskChannel { }

impl terrars::ToListMappable for MskChannel {
    type O = terrars::ListRef<MskChannelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskChannel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_msk_channel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskChannel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel {
    pub fn build(self, stack: &mut terrars::Stack) -> MskChannel {
        let out = MskChannel(std::rc::Rc::new(MskChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskChannelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                channel_name: self.channel_name,
                cluster_arn: self.cluster_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                iceberg_destination: core::default::Default::default(),
                logging_info: core::default::Default::default(),
                s3_destination: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                topic_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskChannelRef {
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

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<MskChannel_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iceberg_destination` after provisioning.\n"]
    pub fn iceberg_destination(&self) -> terrars::ListRef<MskChannel_IcebergDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_info` after provisioning.\n"]
    pub fn logging_info(&self) -> terrars::ListRef<MskChannel_LoggingInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<MskChannel_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskChannel_TimeoutsRef {
        MskChannel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic_configuration` after provisioning.\n"]
    pub fn topic_configuration(&self) -> terrars::ListRef<MskChannel_TopicConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topic_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MskChannelDynamic {
    encryption_configuration: std::option::Option<terrars::DynamicBlock<MskChannel_EncryptionConfiguration>>,
    iceberg_destination: std::option::Option<terrars::DynamicBlock<MskChannel_IcebergDestination>>,
    logging_info: std::option::Option<terrars::DynamicBlock<MskChannel_LoggingInfo>>,
    s3_destination: std::option::Option<terrars::DynamicBlock<MskChannel_S3Destination>>,
    topic_configuration: std::option::Option<terrars::DynamicBlock<MskChannel_TopicConfiguration>>,
}

#[derive(Serialize)]
pub struct MskChannel_EncryptionConfiguration {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl MskChannel_EncryptionConfiguration { }

impl terrars::ToListMappable for MskChannel_EncryptionConfiguration {
    type O = terrars::BlockAssignable<MskChannel_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_EncryptionConfiguration {
    #[doc = ""]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_EncryptionConfiguration {
    pub fn build(self) -> MskChannel_EncryptionConfiguration {
        MskChannel_EncryptionConfiguration { kms_key_arn: self.kms_key_arn }
    }
}

#[allow(dead_code)]
pub struct MskChannel_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_EncryptionConfigurationRef {
        MskChannel_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_Catalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warehouse_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_IcebergDestination_Catalog {
    #[doc = "Set the field `catalog_arn`.\n"]
    pub fn set_catalog_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warehouse_location`.\n"]
    pub fn set_warehouse_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.warehouse_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_Catalog {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_Catalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_Catalog {}

impl BuildMskChannel_IcebergDestination_Catalog {
    pub fn build(self) -> MskChannel_IcebergDestination_Catalog {
        MskChannel_IcebergDestination_Catalog {
            catalog_arn: core::default::Default::default(),
            warehouse_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_CatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_CatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_IcebergDestination_CatalogRef {
        MskChannel_IcebergDestination_CatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_CatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_arn` after provisioning.\n"]
    pub fn catalog_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `warehouse_location` after provisioning.\n"]
    pub fn warehouse_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse_location", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_S3Destination_DeadLetterQueueS3 {
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_S3Destination_DeadLetterQueueS3 {
    #[doc = "Set the field `error_output_prefix`.\n"]
    pub fn set_error_output_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_output_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_S3Destination_DeadLetterQueueS3 {
    type O = terrars::BlockAssignable<MskChannel_S3Destination_DeadLetterQueueS3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_S3Destination_DeadLetterQueueS3 {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_S3Destination_DeadLetterQueueS3 {
    pub fn build(self) -> MskChannel_S3Destination_DeadLetterQueueS3 {
        MskChannel_S3Destination_DeadLetterQueueS3 {
            bucket_arn: self.bucket_arn,
            error_output_prefix: core::default::Default::default(),
            expected_bucket_owner: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_S3Destination_DeadLetterQueueS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_S3Destination_DeadLetterQueueS3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_S3Destination_DeadLetterQueueS3Ref {
        MskChannel_S3Destination_DeadLetterQueueS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_S3Destination_DeadLetterQueueS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `error_output_prefix` after provisioning.\n"]
    pub fn error_output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {}

impl BuildMskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
    pub fn build(self) -> MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
        MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source {
            source_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef {
        MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_IcebergDestination_DestinationTable_PartitionSpecDynamic {
    source: std
    ::option
    ::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source>>,
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_DestinationTable_PartitionSpec {
    partition_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source>>,
    dynamic: MskChannel_IcebergDestination_DestinationTable_PartitionSpecDynamic,
}

impl MskChannel_IcebergDestination_DestinationTable_PartitionSpec {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable_PartitionSpec_Source>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_DestinationTable_PartitionSpec {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable_PartitionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_DestinationTable_PartitionSpec {
    #[doc = ""]
    pub partition_strategy: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_IcebergDestination_DestinationTable_PartitionSpec {
    pub fn build(self) -> MskChannel_IcebergDestination_DestinationTable_PartitionSpec {
        MskChannel_IcebergDestination_DestinationTable_PartitionSpec {
            partition_strategy: self.partition_strategy,
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef {
        MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_strategy` after provisioning.\n"]
    pub fn partition_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<MskChannel_IcebergDestination_DestinationTable_PartitionSpec_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_IcebergDestination_DestinationTableDynamic {
    partition_spec: std
    ::option
    ::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_DestinationTable_PartitionSpec>>,
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_DestinationTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_spec: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_DestinationTable_PartitionSpec>>,
    dynamic: MskChannel_IcebergDestination_DestinationTableDynamic,
}

impl MskChannel_IcebergDestination_DestinationTable {
    #[doc = "Set the field `destination_database_name`.\n"]
    pub fn set_destination_database_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_table_name`.\n"]
    pub fn set_destination_table_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_table_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_spec`.\n"]
    pub fn set_partition_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable_PartitionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partition_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partition_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_DestinationTable {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_DestinationTable {}

impl BuildMskChannel_IcebergDestination_DestinationTable {
    pub fn build(self) -> MskChannel_IcebergDestination_DestinationTable {
        MskChannel_IcebergDestination_DestinationTable {
            destination_database_name: core::default::Default::default(),
            destination_table_name: core::default::Default::default(),
            partition_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_DestinationTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_DestinationTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_IcebergDestination_DestinationTableRef {
        MskChannel_IcebergDestination_DestinationTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_DestinationTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_database_name` after provisioning.\n"]
    pub fn destination_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table_name` after provisioning.\n"]
    pub fn destination_table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(&self) -> terrars::ListRef<MskChannel_IcebergDestination_DestinationTable_PartitionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_SchemaEvolution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_schema_evolution: std::option::Option<terrars::PrimField<bool>>,
}

impl MskChannel_IcebergDestination_SchemaEvolution {
    #[doc = "Set the field `enable_schema_evolution`.\n"]
    pub fn set_enable_schema_evolution(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_schema_evolution = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_SchemaEvolution {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_SchemaEvolution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_SchemaEvolution {}

impl BuildMskChannel_IcebergDestination_SchemaEvolution {
    pub fn build(self) -> MskChannel_IcebergDestination_SchemaEvolution {
        MskChannel_IcebergDestination_SchemaEvolution { enable_schema_evolution: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_SchemaEvolutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_SchemaEvolutionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_IcebergDestination_SchemaEvolutionRef {
        MskChannel_IcebergDestination_SchemaEvolutionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_SchemaEvolutionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_schema_evolution` after provisioning.\n"]
    pub fn enable_schema_evolution(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_schema_evolution", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination_TableCreation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_table_creation: std::option::Option<terrars::PrimField<bool>>,
}

impl MskChannel_IcebergDestination_TableCreation {
    #[doc = "Set the field `enable_table_creation`.\n"]
    pub fn set_enable_table_creation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_table_creation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination_TableCreation {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination_TableCreation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination_TableCreation {}

impl BuildMskChannel_IcebergDestination_TableCreation {
    pub fn build(self) -> MskChannel_IcebergDestination_TableCreation {
        MskChannel_IcebergDestination_TableCreation { enable_table_creation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestination_TableCreationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestination_TableCreationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_IcebergDestination_TableCreationRef {
        MskChannel_IcebergDestination_TableCreationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestination_TableCreationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_table_creation` after provisioning.\n"]
    pub fn enable_table_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_table_creation", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_IcebergDestinationDynamic {
    catalog: std::option::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_Catalog>>,
    dead_letter_queue_s3: std::option::Option<terrars::DynamicBlock<MskChannel_S3Destination_DeadLetterQueueS3>>,
    destination_table: std::option::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_DestinationTable>>,
    schema_evolution: std::option::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_SchemaEvolution>>,
    table_creation: std::option::Option<terrars::DynamicBlock<MskChannel_IcebergDestination_TableCreation>>,
}

#[derive(Serialize)]
pub struct MskChannel_IcebergDestination {
    append_only: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_freshness_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    service_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_Catalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_queue_s3: std::option::Option<std::vec::Vec<MskChannel_S3Destination_DeadLetterQueueS3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_DestinationTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_evolution: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_SchemaEvolution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_creation: std::option::Option<std::vec::Vec<MskChannel_IcebergDestination_TableCreation>>,
    dynamic: MskChannel_IcebergDestinationDynamic,
}

impl MskChannel_IcebergDestination {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_freshness_in_seconds`.\n"]
    pub fn set_data_freshness_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_freshness_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_IcebergDestination_Catalog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dead_letter_queue_s3`.\n"]
    pub fn set_dead_letter_queue_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_S3Destination_DeadLetterQueueS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_queue_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_queue_s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_IcebergDestination_DestinationTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_evolution`.\n"]
    pub fn set_schema_evolution(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_IcebergDestination_SchemaEvolution>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_evolution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_evolution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_creation`.\n"]
    pub fn set_table_creation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_IcebergDestination_TableCreation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_creation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_creation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_IcebergDestination {
    type O = terrars::BlockAssignable<MskChannel_IcebergDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_IcebergDestination {
    #[doc = ""]
    pub append_only: terrars::PrimField<bool>,
    #[doc = ""]
    pub service_execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_IcebergDestination {
    pub fn build(self) -> MskChannel_IcebergDestination {
        MskChannel_IcebergDestination {
            append_only: self.append_only,
            compression_type: core::default::Default::default(),
            data_freshness_in_seconds: core::default::Default::default(),
            service_execution_role_arn: self.service_execution_role_arn,
            catalog: core::default::Default::default(),
            dead_letter_queue_s3: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            schema_evolution: core::default::Default::default(),
            table_creation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_IcebergDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_IcebergDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_IcebergDestinationRef {
        MskChannel_IcebergDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_IcebergDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `append_only` after provisioning.\n"]
    pub fn append_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_only", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_freshness_in_seconds` after provisioning.\n"]
    pub fn data_freshness_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::ListRef<MskChannel_IcebergDestination_CatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue_s3` after provisioning.\n"]
    pub fn dead_letter_queue_s3(&self) -> terrars::ListRef<MskChannel_S3Destination_DeadLetterQueueS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_queue_s3", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> terrars::ListRef<MskChannel_IcebergDestination_DestinationTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_table", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_evolution` after provisioning.\n"]
    pub fn schema_evolution(&self) -> terrars::ListRef<MskChannel_IcebergDestination_SchemaEvolutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_evolution", self.base))
    }

    #[doc = "Get a reference to the value of field `table_creation` after provisioning.\n"]
    pub fn table_creation(&self) -> terrars::ListRef<MskChannel_IcebergDestination_TableCreationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_creation", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_LoggingInfo_CloudwatchLogs {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_LoggingInfo_CloudwatchLogs {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_LoggingInfo_CloudwatchLogs {
    type O = terrars::BlockAssignable<MskChannel_LoggingInfo_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_LoggingInfo_CloudwatchLogs {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskChannel_LoggingInfo_CloudwatchLogs {
    pub fn build(self) -> MskChannel_LoggingInfo_CloudwatchLogs {
        MskChannel_LoggingInfo_CloudwatchLogs {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_LoggingInfo_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_LoggingInfo_CloudwatchLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_LoggingInfo_CloudwatchLogsRef {
        MskChannel_LoggingInfo_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_LoggingInfo_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_LoggingInfo_Firehose {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl MskChannel_LoggingInfo_Firehose {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delivery_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_LoggingInfo_Firehose {
    type O = terrars::BlockAssignable<MskChannel_LoggingInfo_Firehose>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_LoggingInfo_Firehose {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskChannel_LoggingInfo_Firehose {
    pub fn build(self) -> MskChannel_LoggingInfo_Firehose {
        MskChannel_LoggingInfo_Firehose {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_LoggingInfo_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_LoggingInfo_FirehoseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_LoggingInfo_FirehoseRef {
        MskChannel_LoggingInfo_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_LoggingInfo_FirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_LoggingInfo_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_LoggingInfo_S3 {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_LoggingInfo_S3 {
    type O = terrars::BlockAssignable<MskChannel_LoggingInfo_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_LoggingInfo_S3 {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskChannel_LoggingInfo_S3 {
    pub fn build(self) -> MskChannel_LoggingInfo_S3 {
        MskChannel_LoggingInfo_S3 {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_LoggingInfo_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_LoggingInfo_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_LoggingInfo_S3Ref {
        MskChannel_LoggingInfo_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_LoggingInfo_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_LoggingInfoDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<MskChannel_LoggingInfo_CloudwatchLogs>>,
    firehose: std::option::Option<terrars::DynamicBlock<MskChannel_LoggingInfo_Firehose>>,
    s3: std::option::Option<terrars::DynamicBlock<MskChannel_LoggingInfo_S3>>,
}

#[derive(Serialize)]
pub struct MskChannel_LoggingInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<MskChannel_LoggingInfo_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std::option::Option<std::vec::Vec<MskChannel_LoggingInfo_Firehose>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskChannel_LoggingInfo_S3>>,
    dynamic: MskChannel_LoggingInfoDynamic,
}

impl MskChannel_LoggingInfo {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_LoggingInfo_CloudwatchLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_LoggingInfo_Firehose>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_LoggingInfo_S3>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_LoggingInfo {
    type O = terrars::BlockAssignable<MskChannel_LoggingInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_LoggingInfo {}

impl BuildMskChannel_LoggingInfo {
    pub fn build(self) -> MskChannel_LoggingInfo {
        MskChannel_LoggingInfo {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_LoggingInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_LoggingInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_LoggingInfoRef {
        MskChannel_LoggingInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_LoggingInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<MskChannel_LoggingInfo_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> terrars::ListRef<MskChannel_LoggingInfo_FirehoseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskChannel_LoggingInfo_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_S3Destination_Storage {
    bucket_arn: terrars::PrimField<std::string::String>,
    compression_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_key_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl MskChannel_S3Destination_Storage {
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_key_template`.\n"]
    pub fn set_output_key_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_key_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_prefix`.\n"]
    pub fn set_output_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_S3Destination_Storage {
    type O = terrars::BlockAssignable<MskChannel_S3Destination_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_S3Destination_Storage {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub compression_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_S3Destination_Storage {
    pub fn build(self) -> MskChannel_S3Destination_Storage {
        MskChannel_S3Destination_Storage {
            bucket_arn: self.bucket_arn,
            compression_type: self.compression_type,
            expected_bucket_owner: core::default::Default::default(),
            output_key_template: core::default::Default::default(),
            output_prefix: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_S3Destination_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_S3Destination_StorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_S3Destination_StorageRef {
        MskChannel_S3Destination_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_S3Destination_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `output_key_template` after provisioning.\n"]
    pub fn output_key_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_key_template", self.base))
    }

    #[doc = "Get a reference to the value of field `output_prefix` after provisioning.\n"]
    pub fn output_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_S3DestinationDynamic {
    dead_letter_queue_s3: std::option::Option<terrars::DynamicBlock<MskChannel_S3Destination_DeadLetterQueueS3>>,
    storage: std::option::Option<terrars::DynamicBlock<MskChannel_S3Destination_Storage>>,
}

#[derive(Serialize)]
pub struct MskChannel_S3Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_freshness_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    service_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_queue_s3: std::option::Option<std::vec::Vec<MskChannel_S3Destination_DeadLetterQueueS3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<std::vec::Vec<MskChannel_S3Destination_Storage>>,
    dynamic: MskChannel_S3DestinationDynamic,
}

impl MskChannel_S3Destination {
    #[doc = "Set the field `data_freshness_in_seconds`.\n"]
    pub fn set_data_freshness_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_freshness_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_queue_s3`.\n"]
    pub fn set_dead_letter_queue_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_S3Destination_DeadLetterQueueS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_queue_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_queue_s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_S3Destination_Storage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_S3Destination {
    type O = terrars::BlockAssignable<MskChannel_S3Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_S3Destination {
    #[doc = ""]
    pub service_execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_S3Destination {
    pub fn build(self) -> MskChannel_S3Destination {
        MskChannel_S3Destination {
            data_freshness_in_seconds: core::default::Default::default(),
            service_execution_role_arn: self.service_execution_role_arn,
            dead_letter_queue_s3: core::default::Default::default(),
            storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_S3DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_S3DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_S3DestinationRef {
        MskChannel_S3DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_S3DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_freshness_in_seconds` after provisioning.\n"]
    pub fn data_freshness_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue_s3` after provisioning.\n"]
    pub fn dead_letter_queue_s3(&self) -> terrars::ListRef<MskChannel_S3Destination_DeadLetterQueueS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_queue_s3", self.base))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(&self) -> terrars::ListRef<MskChannel_S3Destination_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskChannel_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskChannel_Timeouts {
    type O = terrars::BlockAssignable<MskChannel_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_Timeouts {}

impl BuildMskChannel_Timeouts {
    pub fn build(self) -> MskChannel_Timeouts {
        MskChannel_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_TimeoutsRef {
        MskChannel_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_TopicConfiguration_RecordConverter {
    value_converter: terrars::PrimField<std::string::String>,
}

impl MskChannel_TopicConfiguration_RecordConverter { }

impl terrars::ToListMappable for MskChannel_TopicConfiguration_RecordConverter {
    type O = terrars::BlockAssignable<MskChannel_TopicConfiguration_RecordConverter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_TopicConfiguration_RecordConverter {
    #[doc = ""]
    pub value_converter: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_TopicConfiguration_RecordConverter {
    pub fn build(self) -> MskChannel_TopicConfiguration_RecordConverter {
        MskChannel_TopicConfiguration_RecordConverter { value_converter: self.value_converter }
    }
}

#[allow(dead_code)]
pub struct MskChannel_TopicConfiguration_RecordConverterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_TopicConfiguration_RecordConverterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskChannel_TopicConfiguration_RecordConverterRef {
        MskChannel_TopicConfiguration_RecordConverterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_TopicConfiguration_RecordConverterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value_converter` after provisioning.\n"]
    pub fn value_converter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_converter", self.base))
    }
}

#[derive(Serialize)]
pub struct MskChannel_TopicConfiguration_RecordSchema {
    gsr_arn: terrars::PrimField<std::string::String>,
}

impl MskChannel_TopicConfiguration_RecordSchema { }

impl terrars::ToListMappable for MskChannel_TopicConfiguration_RecordSchema {
    type O = terrars::BlockAssignable<MskChannel_TopicConfiguration_RecordSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_TopicConfiguration_RecordSchema {
    #[doc = ""]
    pub gsr_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_TopicConfiguration_RecordSchema {
    pub fn build(self) -> MskChannel_TopicConfiguration_RecordSchema {
        MskChannel_TopicConfiguration_RecordSchema { gsr_arn: self.gsr_arn }
    }
}

#[allow(dead_code)]
pub struct MskChannel_TopicConfiguration_RecordSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_TopicConfiguration_RecordSchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_TopicConfiguration_RecordSchemaRef {
        MskChannel_TopicConfiguration_RecordSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_TopicConfiguration_RecordSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gsr_arn` after provisioning.\n"]
    pub fn gsr_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gsr_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskChannel_TopicConfigurationDynamic {
    record_converter: std::option::Option<terrars::DynamicBlock<MskChannel_TopicConfiguration_RecordConverter>>,
    record_schema: std::option::Option<terrars::DynamicBlock<MskChannel_TopicConfiguration_RecordSchema>>,
}

#[derive(Serialize)]
pub struct MskChannel_TopicConfiguration {
    topic_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_converter: std::option::Option<std::vec::Vec<MskChannel_TopicConfiguration_RecordConverter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_schema: std::option::Option<std::vec::Vec<MskChannel_TopicConfiguration_RecordSchema>>,
    dynamic: MskChannel_TopicConfigurationDynamic,
}

impl MskChannel_TopicConfiguration {
    #[doc = "Set the field `record_converter`.\n"]
    pub fn set_record_converter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_TopicConfiguration_RecordConverter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_converter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_converter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_schema`.\n"]
    pub fn set_record_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskChannel_TopicConfiguration_RecordSchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskChannel_TopicConfiguration {
    type O = terrars::BlockAssignable<MskChannel_TopicConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskChannel_TopicConfiguration {
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildMskChannel_TopicConfiguration {
    pub fn build(self) -> MskChannel_TopicConfiguration {
        MskChannel_TopicConfiguration {
            topic_arn: self.topic_arn,
            record_converter: core::default::Default::default(),
            record_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskChannel_TopicConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskChannel_TopicConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskChannel_TopicConfigurationRef {
        MskChannel_TopicConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskChannel_TopicConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `record_converter` after provisioning.\n"]
    pub fn record_converter(&self) -> terrars::ListRef<MskChannel_TopicConfiguration_RecordConverterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_converter", self.base))
    }

    #[doc = "Get a reference to the value of field `record_schema` after provisioning.\n"]
    pub fn record_schema(&self) -> terrars::ListRef<MskChannel_TopicConfiguration_RecordSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_schema", self.base))
    }
}
