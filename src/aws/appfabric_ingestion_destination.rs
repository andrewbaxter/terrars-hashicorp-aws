#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppfabricIngestionDestinationData_ {
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
    app_bundle_arn: terrars::PrimField<std::string::String>,
    ingestion_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configuration: std::option::Option<std::vec::Vec<AppfabricIngestionDestination_DestinationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_configuration: std::option::Option<std::vec::Vec<AppfabricIngestionDestination_ProcessingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppfabricIngestionDestination_Timeouts>,
    dynamic: AppfabricIngestionDestinationDynamic,
}

struct AppfabricIngestionDestination_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppfabricIngestionDestinationData_>,
}

#[derive(Clone)]
pub struct AppfabricIngestionDestination(std::rc::Rc<AppfabricIngestionDestination_>);

impl AppfabricIngestionDestination {
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

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricIngestionDestination_ProcessingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppfabricIngestionDestination_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingestion_arn` after provisioning.\n"]
    pub fn ingestion_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> terrars::ListRef<AppfabricIngestionDestination_DestinationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(&self) -> terrars::ListRef<AppfabricIngestionDestination_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricIngestionDestination_TimeoutsRef {
        AppfabricIngestionDestination_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AppfabricIngestionDestination {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppfabricIngestionDestination { }

impl terrars::ToListMappable for AppfabricIngestionDestination {
    type O = terrars::ListRef<AppfabricIngestionDestinationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppfabricIngestionDestination_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appfabric_ingestion_destination".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppfabricIngestionDestination {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_bundle_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ingestion_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricIngestionDestination {
    pub fn build(self, stack: &mut terrars::Stack) -> AppfabricIngestionDestination {
        let out = AppfabricIngestionDestination(std::rc::Rc::new(AppfabricIngestionDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppfabricIngestionDestinationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_bundle_arn: self.app_bundle_arn,
                ingestion_arn: self.ingestion_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                processing_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppfabricIngestionDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppfabricIngestionDestinationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingestion_arn` after provisioning.\n"]
    pub fn ingestion_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> terrars::ListRef<AppfabricIngestionDestination_DestinationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(&self) -> terrars::ListRef<AppfabricIngestionDestination_ProcessingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.processing_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricIngestionDestination_TimeoutsRef {
        AppfabricIngestionDestination_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AppfabricIngestionDestinationDynamic {
    destination_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricIngestionDestination_DestinationConfiguration>>,
    processing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricIngestionDestination_ProcessingConfiguration>>,
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
    stream_name: terrars::PrimField<std::string::String>,
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream { }

impl terrars
::ToListMappable for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
    type O =
        terrars
        ::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
    #[doc = ""]
    pub stream_name: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
    pub fn build(self) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream {
            stream_name: self.stream_name,
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
    type O =
        terrars
        ::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
    pub fn build(self) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket {
            bucket_name: self.bucket_name,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationDynamic {
    firehose_stream: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream>,
    >,
    s3_bucket: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket>,
    >,
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose_stream: std
    ::option
    ::Option<
        std::vec::Vec<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std
    ::option
    ::Option<std::vec::Vec<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket>>,
    dynamic: AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationDynamic,
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
    #[doc = "Set the field `firehose_stream`.\n"]
    pub fn set_firehose_stream(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStream,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose_stream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_stream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3Bucket,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {}

impl BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
    pub fn build(self) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination {
            firehose_stream: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firehose_stream` after provisioning.\n"]
    pub fn firehose_stream(
        &self,
    ) -> terrars
    ::ListRef<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_FirehoseStreamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(
        &self,
    ) -> terrars::ListRef<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination_S3BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricIngestionDestination_DestinationConfiguration_AuditLogDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination>>,
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination>>,
    dynamic: AppfabricIngestionDestination_DestinationConfiguration_AuditLogDynamic,
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLog {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_Destination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppfabricIngestionDestination_DestinationConfiguration_AuditLog {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog {}

impl BuildAppfabricIngestionDestination_DestinationConfiguration_AuditLog {
    pub fn build(self) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLog {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLog {
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef {
        AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<AppfabricIngestionDestination_DestinationConfiguration_AuditLog_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricIngestionDestination_DestinationConfigurationDynamic {
    audit_log: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricIngestionDestination_DestinationConfiguration_AuditLog>>,
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_DestinationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log: std::option::Option<std::vec::Vec<AppfabricIngestionDestination_DestinationConfiguration_AuditLog>>,
    dynamic: AppfabricIngestionDestination_DestinationConfigurationDynamic,
}

impl AppfabricIngestionDestination_DestinationConfiguration {
    #[doc = "Set the field `audit_log`.\n"]
    pub fn set_audit_log(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration_AuditLog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit_log = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppfabricIngestionDestination_DestinationConfiguration {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_DestinationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_DestinationConfiguration {}

impl BuildAppfabricIngestionDestination_DestinationConfiguration {
    pub fn build(self) -> AppfabricIngestionDestination_DestinationConfiguration {
        AppfabricIngestionDestination_DestinationConfiguration {
            audit_log: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_DestinationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_DestinationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_DestinationConfigurationRef {
        AppfabricIngestionDestination_DestinationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_DestinationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log` after provisioning.\n"]
    pub fn audit_log(&self) -> terrars::ListRef<AppfabricIngestionDestination_DestinationConfiguration_AuditLogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_log", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
    format: terrars::PrimField<std::string::String>,
    schema: terrars::PrimField<std::string::String>,
}

impl AppfabricIngestionDestination_ProcessingConfiguration_AuditLog { }

impl terrars::ToListMappable for AppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_ProcessingConfiguration_AuditLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
    pub fn build(self) -> AppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
        AppfabricIngestionDestination_ProcessingConfiguration_AuditLog {
            format: self.format,
            schema: self.schema,
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef {
        AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricIngestionDestination_ProcessingConfigurationDynamic {
    audit_log: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricIngestionDestination_ProcessingConfiguration_AuditLog>>,
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_ProcessingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log: std::option::Option<std::vec::Vec<AppfabricIngestionDestination_ProcessingConfiguration_AuditLog>>,
    dynamic: AppfabricIngestionDestination_ProcessingConfigurationDynamic,
}

impl AppfabricIngestionDestination_ProcessingConfiguration {
    #[doc = "Set the field `audit_log`.\n"]
    pub fn set_audit_log(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppfabricIngestionDestination_ProcessingConfiguration_AuditLog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit_log = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppfabricIngestionDestination_ProcessingConfiguration {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_ProcessingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_ProcessingConfiguration {}

impl BuildAppfabricIngestionDestination_ProcessingConfiguration {
    pub fn build(self) -> AppfabricIngestionDestination_ProcessingConfiguration {
        AppfabricIngestionDestination_ProcessingConfiguration {
            audit_log: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_ProcessingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_ProcessingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricIngestionDestination_ProcessingConfigurationRef {
        AppfabricIngestionDestination_ProcessingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_ProcessingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_log` after provisioning.\n"]
    pub fn audit_log(&self) -> terrars::ListRef<AppfabricIngestionDestination_ProcessingConfiguration_AuditLogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_log", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricIngestionDestination_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppfabricIngestionDestination_Timeouts {
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

impl terrars::ToListMappable for AppfabricIngestionDestination_Timeouts {
    type O = terrars::BlockAssignable<AppfabricIngestionDestination_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricIngestionDestination_Timeouts {}

impl BuildAppfabricIngestionDestination_Timeouts {
    pub fn build(self) -> AppfabricIngestionDestination_Timeouts {
        AppfabricIngestionDestination_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricIngestionDestination_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricIngestionDestination_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppfabricIngestionDestination_TimeoutsRef {
        AppfabricIngestionDestination_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricIngestionDestination_TimeoutsRef {
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
