#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlBucketLifecycleConfigurationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3controlBucketLifecycleConfiguration_Rule>>,
    dynamic: S3controlBucketLifecycleConfigurationDynamic,
}

struct S3controlBucketLifecycleConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlBucketLifecycleConfigurationData_>,
}

#[derive(Clone)]
pub struct S3controlBucketLifecycleConfiguration(std::rc::Rc<S3controlBucketLifecycleConfiguration_>);

impl S3controlBucketLifecycleConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for S3controlBucketLifecycleConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlBucketLifecycleConfiguration { }

impl terrars::ToListMappable for S3controlBucketLifecycleConfiguration {
    type O = terrars::ListRef<S3controlBucketLifecycleConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlBucketLifecycleConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_bucket_lifecycle_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3controlBucketLifecycleConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlBucketLifecycleConfiguration {
        let out = S3controlBucketLifecycleConfiguration(std::rc::Rc::new(S3controlBucketLifecycleConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlBucketLifecycleConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlBucketLifecycleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlBucketLifecycleConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlBucketLifecycleConfigurationRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3controlBucketLifecycleConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3controlBucketLifecycleConfiguration_Rule>>,
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    days_after_initiation: terrars::PrimField<f64>,
}

impl S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload { }

impl terrars::ToListMappable for S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    type O = terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    #[doc = ""]
    pub days_after_initiation: terrars::PrimField<f64>,
}

impl BuildS3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    pub fn build(self) -> S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
        S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
            days_after_initiation: self.days_after_initiation,
        }
    }
}

#[allow(dead_code)]
pub struct S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
        S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_after_initiation` after provisioning.\n"]
    pub fn days_after_initiation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_after_initiation", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfiguration_Rule_Expiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expired_object_delete_marker: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlBucketLifecycleConfiguration_Rule_Expiration {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expired_object_delete_marker`.\n"]
    pub fn set_expired_object_delete_marker(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.expired_object_delete_marker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlBucketLifecycleConfiguration_Rule_Expiration {
    type O = terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_Expiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration_Rule_Expiration {}

impl BuildS3controlBucketLifecycleConfiguration_Rule_Expiration {
    pub fn build(self) -> S3controlBucketLifecycleConfiguration_Rule_Expiration {
        S3controlBucketLifecycleConfiguration_Rule_Expiration {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlBucketLifecycleConfiguration_Rule_ExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlBucketLifecycleConfiguration_Rule_ExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlBucketLifecycleConfiguration_Rule_ExpirationRef {
        S3controlBucketLifecycleConfiguration_Rule_ExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfiguration_Rule_ExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expired_object_delete_marker` after provisioning.\n"]
    pub fn expired_object_delete_marker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expired_object_delete_marker", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfiguration_Rule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3controlBucketLifecycleConfiguration_Rule_Filter {
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

impl terrars::ToListMappable for S3controlBucketLifecycleConfiguration_Rule_Filter {
    type O = terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration_Rule_Filter {}

impl BuildS3controlBucketLifecycleConfiguration_Rule_Filter {
    pub fn build(self) -> S3controlBucketLifecycleConfiguration_Rule_Filter {
        S3controlBucketLifecycleConfiguration_Rule_Filter {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlBucketLifecycleConfiguration_Rule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlBucketLifecycleConfiguration_Rule_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlBucketLifecycleConfiguration_Rule_FilterRef {
        S3controlBucketLifecycleConfiguration_Rule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfiguration_Rule_FilterRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct S3controlBucketLifecycleConfiguration_RuleDynamic {
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>>,
    expiration: std::option::Option<terrars::DynamicBlock<S3controlBucketLifecycleConfiguration_Rule_Expiration>>,
    filter: std::option::Option<terrars::DynamicBlock<S3controlBucketLifecycleConfiguration_Rule_Filter>>,
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfiguration_Rule {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<std::vec::Vec<S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<std::vec::Vec<S3controlBucketLifecycleConfiguration_Rule_Expiration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3controlBucketLifecycleConfiguration_Rule_Filter>>,
    dynamic: S3controlBucketLifecycleConfiguration_RuleDynamic,
}

impl S3controlBucketLifecycleConfiguration_Rule {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `abort_incomplete_multipart_upload`.\n"]
    pub fn set_abort_incomplete_multipart_upload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.abort_incomplete_multipart_upload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.abort_incomplete_multipart_upload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_Expiration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlBucketLifecycleConfiguration_Rule {
    type O = terrars::BlockAssignable<S3controlBucketLifecycleConfiguration_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration_Rule {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildS3controlBucketLifecycleConfiguration_Rule {
    pub fn build(self) -> S3controlBucketLifecycleConfiguration_Rule {
        S3controlBucketLifecycleConfiguration_Rule {
            id: self.id,
            status: core::default::Default::default(),
            abort_incomplete_multipart_upload: core::default::Default::default(),
            expiration: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlBucketLifecycleConfiguration_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlBucketLifecycleConfiguration_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3controlBucketLifecycleConfiguration_RuleRef {
        S3controlBucketLifecycleConfiguration_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfiguration_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload(
        &self,
    ) -> terrars::ListRef<S3controlBucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::ListRef<S3controlBucketLifecycleConfiguration_Rule_ExpirationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3controlBucketLifecycleConfiguration_Rule_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
