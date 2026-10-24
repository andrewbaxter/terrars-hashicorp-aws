use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketAnalyticsConfigurationData {
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
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketAnalyticsConfigurationFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class_analysis: std::option::Option<std::vec::Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
    dynamic: S3BucketAnalyticsConfigurationDynamic,
}

struct S3BucketAnalyticsConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketAnalyticsConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketAnalyticsConfiguration(std::rc::Rc<S3BucketAnalyticsConfiguration_>);

impl S3BucketAnalyticsConfiguration {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAnalyticsConfigurationFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_class_analysis`.\n"]
    pub fn set_storage_class_analysis(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_class_analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_class_analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketAnalyticsConfigurationFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_class_analysis` after provisioning.\n"]
    pub fn storage_class_analysis(&self) -> terrars::ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_class_analysis", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketAnalyticsConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketAnalyticsConfiguration { }

impl terrars::ToListMappable for S3BucketAnalyticsConfiguration {
    type O = terrars::ListRef<S3BucketAnalyticsConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketAnalyticsConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_analytics_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketAnalyticsConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAnalyticsConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketAnalyticsConfiguration {
        let out = S3BucketAnalyticsConfiguration(std::rc::Rc::new(S3BucketAnalyticsConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketAnalyticsConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                id: core::default::Default::default(),
                name: self.name,
                filter: core::default::Default::default(),
                storage_class_analysis: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketAnalyticsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAnalyticsConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketAnalyticsConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketAnalyticsConfigurationFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_class_analysis` after provisioning.\n"]
    pub fn storage_class_analysis(&self) -> terrars::ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_class_analysis", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3BucketAnalyticsConfigurationFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketAnalyticsConfigurationFilterEl {
    type O = terrars::BlockAssignable<S3BucketAnalyticsConfigurationFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationFilterEl {}

impl BuildS3BucketAnalyticsConfigurationFilterEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationFilterEl {
        S3BucketAnalyticsConfigurationFilterEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAnalyticsConfigurationFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketAnalyticsConfigurationFilterElRef {
        S3BucketAnalyticsConfigurationFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    #[doc = "Set the field `bucket_account_id`.\n"]
    pub fn set_bucket_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
    pub fn build(
        self,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl {
            bucket_account_id: core::default::Default::default(),
            bucket_arn: self.bucket_arn,
            format: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElDynamic {
    s3_bucket_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl>,
    >,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    #[doc = "Set the field `s3_bucket_destination`.\n"]
    pub fn set_s3_bucket_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    type O = terrars::BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl {
            s3_bucket_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_destination` after provisioning.\n"]
    pub fn s3_bucket_destination(
        &self,
    ) -> terrars
    ::ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElS3BucketDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl>>,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl>>,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    #[doc = "Set the field `output_schema_version`.\n"]
    pub fn set_output_schema_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_schema_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationEl,
                        >,
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

impl terrars::ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    type O = terrars::BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl {
            output_schema_version: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_schema_version` after provisioning.\n"]
    pub fn output_schema_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema_version", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationStorageClassAnalysisElDynamic {
    data_export: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>>,
}

#[derive(Serialize)]
pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_export: std::option::Option<std::vec::Vec<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>>,
    dynamic: S3BucketAnalyticsConfigurationStorageClassAnalysisElDynamic,
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    #[doc = "Set the field `data_export`.\n"]
    pub fn set_data_export(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_export = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    type O = terrars::BlockAssignable<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAnalyticsConfigurationStorageClassAnalysisEl {}

impl BuildS3BucketAnalyticsConfigurationStorageClassAnalysisEl {
    pub fn build(self) -> S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
        S3BucketAnalyticsConfigurationStorageClassAnalysisEl {
            data_export: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
        S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAnalyticsConfigurationStorageClassAnalysisElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_export` after provisioning.\n"]
    pub fn data_export(&self) -> terrars::ListRef<S3BucketAnalyticsConfigurationStorageClassAnalysisElDataExportElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_export", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAnalyticsConfigurationDynamic {
    filter: std::option::Option<terrars::DynamicBlock<S3BucketAnalyticsConfigurationFilterEl>>,
    storage_class_analysis: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketAnalyticsConfigurationStorageClassAnalysisEl>>,
}
