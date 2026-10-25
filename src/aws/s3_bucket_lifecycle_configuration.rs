#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketLifecycleConfigurationData_ {
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
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_default_minimum_object_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<S3BucketLifecycleConfiguration_Timeouts>,
    dynamic: S3BucketLifecycleConfigurationDynamic,
}

struct S3BucketLifecycleConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketLifecycleConfigurationData_>,
}

#[derive(Clone)]
pub struct S3BucketLifecycleConfiguration(std::rc::Rc<S3BucketLifecycleConfiguration_>);

impl S3BucketLifecycleConfiguration {
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

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transition_default_minimum_object_size`.\n"]
    pub fn set_transition_default_minimum_object_size(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transition_default_minimum_object_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<S3BucketLifecycleConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transition_default_minimum_object_size` after provisioning.\n"]
    pub fn transition_default_minimum_object_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transition_default_minimum_object_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketLifecycleConfiguration_TimeoutsRef {
        S3BucketLifecycleConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for S3BucketLifecycleConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketLifecycleConfiguration { }

impl terrars::ToListMappable for S3BucketLifecycleConfiguration {
    type O = terrars::ListRef<S3BucketLifecycleConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketLifecycleConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_lifecycle_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketLifecycleConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketLifecycleConfiguration {
        let out = S3BucketLifecycleConfiguration(std::rc::Rc::new(S3BucketLifecycleConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketLifecycleConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                region: core::default::Default::default(),
                transition_default_minimum_object_size: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketLifecycleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketLifecycleConfigurationRef {
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

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transition_default_minimum_object_size` after provisioning.\n"]
    pub fn transition_default_minimum_object_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transition_default_minimum_object_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketLifecycleConfiguration_TimeoutsRef {
        S3BucketLifecycleConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct S3BucketLifecycleConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_after_initiation: std::option::Option<terrars::PrimField<f64>>,
}

impl S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    #[doc = "Set the field `days_after_initiation`.\n"]
    pub fn set_days_after_initiation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_after_initiation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {}

impl BuildS3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
        S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload {
            days_after_initiation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
        S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef {
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
pub struct S3BucketLifecycleConfiguration_Rule_Expiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expired_object_delete_marker: std::option::Option<terrars::PrimField<bool>>,
}

impl S3BucketLifecycleConfiguration_Rule_Expiration {
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

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_Expiration {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Expiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_Expiration {}

impl BuildS3BucketLifecycleConfiguration_Rule_Expiration {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_Expiration {
        S3BucketLifecycleConfiguration_Rule_Expiration {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_ExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_ExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_ExpirationRef {
        S3BucketLifecycleConfiguration_Rule_ExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_ExpirationRef {
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
pub struct S3BucketLifecycleConfiguration_Rule_Filter_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_greater_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_less_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3BucketLifecycleConfiguration_Rule_Filter_And {
    #[doc = "Set the field `object_size_greater_than`.\n"]
    pub fn set_object_size_greater_than(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.object_size_greater_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_size_less_than`.\n"]
    pub fn set_object_size_less_than(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.object_size_less_than = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_Filter_And {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_Filter_And {}

impl BuildS3BucketLifecycleConfiguration_Rule_Filter_And {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_Filter_And {
        S3BucketLifecycleConfiguration_Rule_Filter_And {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_Filter_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_Filter_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_Filter_AndRef {
        S3BucketLifecycleConfiguration_Rule_Filter_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_Filter_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_size_greater_than` after provisioning.\n"]
    pub fn object_size_greater_than(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_greater_than", self.base))
    }

    #[doc = "Get a reference to the value of field `object_size_less_than` after provisioning.\n"]
    pub fn object_size_less_than(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_less_than", self.base))
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
pub struct S3BucketLifecycleConfiguration_Rule_Filter_Tag {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfiguration_Rule_Filter_Tag { }

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_Filter_Tag {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_Filter_Tag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfiguration_Rule_Filter_Tag {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_Filter_Tag {
        S3BucketLifecycleConfiguration_Rule_Filter_Tag {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_Filter_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_Filter_TagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_Filter_TagRef {
        S3BucketLifecycleConfiguration_Rule_Filter_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_Filter_TagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLifecycleConfiguration_Rule_FilterDynamic {
    and: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_Filter_And>>,
    tag: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_Filter_Tag>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_greater_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_less_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_Filter_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_Filter_Tag>>,
    dynamic: S3BucketLifecycleConfiguration_Rule_FilterDynamic,
}

impl S3BucketLifecycleConfiguration_Rule_Filter {
    #[doc = "Set the field `object_size_greater_than`.\n"]
    pub fn set_object_size_greater_than(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.object_size_greater_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_size_less_than`.\n"]
    pub fn set_object_size_less_than(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.object_size_less_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter_Tag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_Filter {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_Filter {}

impl BuildS3BucketLifecycleConfiguration_Rule_Filter {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_Filter {
        S3BucketLifecycleConfiguration_Rule_Filter {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLifecycleConfiguration_Rule_FilterRef {
        S3BucketLifecycleConfiguration_Rule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_size_greater_than` after provisioning.\n"]
    pub fn object_size_greater_than(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_greater_than", self.base))
    }

    #[doc = "Get a reference to the value of field `object_size_less_than` after provisioning.\n"]
    pub fn object_size_less_than(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_less_than", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_Filter_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_Filter_TagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    newer_noncurrent_versions: std::option::Option<terrars::PrimField<f64>>,
    noncurrent_days: terrars::PrimField<f64>,
}

impl S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.newer_noncurrent_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
    #[doc = ""]
    pub noncurrent_days: terrars::PrimField<f64>,
}

impl BuildS3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
        S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: self.noncurrent_days,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef {
        S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.newer_noncurrent_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_days` after provisioning.\n"]
    pub fn noncurrent_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_days", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    newer_noncurrent_versions: std::option::Option<terrars::PrimField<f64>>,
    noncurrent_days: terrars::PrimField<f64>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.newer_noncurrent_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
    #[doc = ""]
    pub noncurrent_days: terrars::PrimField<f64>,
    #[doc = ""]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
        S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: self.noncurrent_days,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransitionRef {
        S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.newer_noncurrent_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_days` after provisioning.\n"]
    pub fn noncurrent_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_days", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule_Transition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfiguration_Rule_Transition {
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
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule_Transition {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Transition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule_Transition {
    #[doc = ""]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfiguration_Rule_Transition {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule_Transition {
        S3BucketLifecycleConfiguration_Rule_Transition {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_Rule_TransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_Rule_TransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfiguration_Rule_TransitionRef {
        S3BucketLifecycleConfiguration_Rule_TransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_Rule_TransitionRef {
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

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLifecycleConfiguration_RuleDynamic {
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>>,
    expiration: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_Expiration>>,
    filter: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_Filter>>,
    noncurrent_version_expiration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration>>,
    noncurrent_version_transition: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition>>,
    transition: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfiguration_Rule_Transition>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Rule {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_Expiration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_version_expiration: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_version_transition: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition: std::option::Option<std::vec::Vec<S3BucketLifecycleConfiguration_Rule_Transition>>,
    dynamic: S3BucketLifecycleConfiguration_RuleDynamic,
}

impl S3BucketLifecycleConfiguration_Rule {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUpload>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Expiration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Filter>>,
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

    #[doc = "Set the field `noncurrent_version_expiration`.\n"]
    pub fn set_noncurrent_version_expiration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpiration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.noncurrent_version_expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_expiration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `noncurrent_version_transition`.\n"]
    pub fn set_noncurrent_version_transition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionTransition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.noncurrent_version_transition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_transition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transition`.\n"]
    pub fn set_transition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule_Transition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Rule {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Rule {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfiguration_Rule {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Rule {
        S3BucketLifecycleConfiguration_Rule {
            id: self.id,
            prefix: core::default::Default::default(),
            status: self.status,
            abort_incomplete_multipart_upload: core::default::Default::default(),
            expiration: core::default::Default::default(),
            filter: core::default::Default::default(),
            noncurrent_version_expiration: core::default::Default::default(),
            noncurrent_version_transition: core::default::Default::default(),
            transition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLifecycleConfiguration_RuleRef {
        S3BucketLifecycleConfiguration_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload(
        &self,
    ) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_AbortIncompleteMultipartUploadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_ExpirationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_version_expiration` after provisioning.\n"]
    pub fn noncurrent_version_expiration(
        &self,
    ) -> terrars::ListRef<S3BucketLifecycleConfiguration_Rule_NoncurrentVersionExpirationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.noncurrent_version_expiration", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketLifecycleConfiguration_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfiguration_Timeouts {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfiguration_Timeouts {}

impl BuildS3BucketLifecycleConfiguration_Timeouts {
    pub fn build(self) -> S3BucketLifecycleConfiguration_Timeouts {
        S3BucketLifecycleConfiguration_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLifecycleConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfiguration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLifecycleConfiguration_TimeoutsRef {
        S3BucketLifecycleConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfiguration_TimeoutsRef {
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
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
