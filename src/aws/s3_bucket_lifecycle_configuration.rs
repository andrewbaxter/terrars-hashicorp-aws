use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketLifecycleConfigurationData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleEl>>,
    dynamic: S3BucketLifecycleConfigurationDynamic,
}

struct S3BucketLifecycleConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketLifecycleConfigurationData>,
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleEl>>,
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

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
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
            data: std::cell::RefCell::new(S3BucketLifecycleConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                rule: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_after_initiation: std::option::Option<terrars::PrimField<f64>>,
}

impl S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    #[doc = "Set the field `days_after_initiation`.\n"]
    pub fn set_days_after_initiation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_after_initiation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {}

impl BuildS3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
        S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
            days_after_initiation: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
        S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_after_initiation` after provisioning.\n"]
    pub fn days_after_initiation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_after_initiation", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElExpirationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expired_object_delete_marker: std::option::Option<terrars::PrimField<bool>>,
}

impl S3BucketLifecycleConfigurationRuleElExpirationEl {
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

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElExpirationEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElExpirationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElExpirationEl {}

impl BuildS3BucketLifecycleConfigurationRuleElExpirationEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElExpirationEl {
        S3BucketLifecycleConfigurationRuleElExpirationEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElExpirationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElExpirationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElExpirationElRef {
        S3BucketLifecycleConfigurationRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElExpirationElRef {
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
pub struct S3BucketLifecycleConfigurationRuleElFilterElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_greater_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_less_than: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3BucketLifecycleConfigurationRuleElFilterElAndEl {
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

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElFilterElAndEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElFilterElAndEl {}

impl BuildS3BucketLifecycleConfigurationRuleElFilterElAndEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterElAndEl {
        S3BucketLifecycleConfigurationRuleElFilterElAndEl {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
        S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
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
pub struct S3BucketLifecycleConfigurationRuleElFilterElTagEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfigurationRuleElFilterElTagEl { }

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElFilterElTagEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElFilterElTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfigurationRuleElFilterElTagEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterElTagEl {
        S3BucketLifecycleConfigurationRuleElFilterElTagEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
        S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
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
struct S3BucketLifecycleConfigurationRuleElFilterElDynamic {
    and: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
    tag: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_greater_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_size_less_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
    dynamic: S3BucketLifecycleConfigurationRuleElFilterElDynamic,
}

impl S3BucketLifecycleConfigurationRuleElFilterEl {
    #[doc = "Set the field `object_size_greater_than`.\n"]
    pub fn set_object_size_greater_than(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.object_size_greater_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_size_less_than`.\n"]
    pub fn set_object_size_less_than(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
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

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElFilterEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElFilterEl {}

impl BuildS3BucketLifecycleConfigurationRuleElFilterEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterEl {
        S3BucketLifecycleConfigurationRuleElFilterEl {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElFilterElRef {
        S3BucketLifecycleConfigurationRuleElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_size_greater_than` after provisioning.\n"]
    pub fn object_size_greater_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_greater_than", self.base))
    }

    #[doc = "Get a reference to the value of field `object_size_less_than` after provisioning.\n"]
    pub fn object_size_less_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_size_less_than", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElFilterElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElFilterElTagElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    newer_noncurrent_versions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_days: std::option::Option<terrars::PrimField<f64>>,
}

impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.newer_noncurrent_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `noncurrent_days`.\n"]
    pub fn set_noncurrent_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.noncurrent_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {}

impl BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.newer_noncurrent_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_days` after provisioning.\n"]
    pub fn noncurrent_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noncurrent_days", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    newer_noncurrent_versions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_days: std::option::Option<terrars::PrimField<f64>>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.newer_noncurrent_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `noncurrent_days`.\n"]
    pub fn set_noncurrent_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.noncurrent_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[doc = ""]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> terrars::PrimExpr<std::string::String> {
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
pub struct S3BucketLifecycleConfigurationRuleElTransitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    storage_class: terrars::PrimField<std::string::String>,
}

impl S3BucketLifecycleConfigurationRuleElTransitionEl {
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

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleElTransitionEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElTransitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleElTransitionEl {
    #[doc = ""]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfigurationRuleElTransitionEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElTransitionEl {
        S3BucketLifecycleConfigurationRuleElTransitionEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

pub struct S3BucketLifecycleConfigurationRuleElTransitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElTransitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLifecycleConfigurationRuleElTransitionElRef {
        S3BucketLifecycleConfigurationRuleElTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElTransitionElRef {
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
struct S3BucketLifecycleConfigurationRuleElDynamic {
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>>,
    expiration: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
    filter: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterEl>>,
    noncurrent_version_expiration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>>,
    noncurrent_version_transition: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>>,
    transition: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_incomplete_multipart_upload: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_version_expiration: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_version_transition: std
    ::option
    ::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition: std::option::Option<std::vec::Vec<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
    dynamic: S3BucketLifecycleConfigurationRuleElDynamic,
}

impl S3BucketLifecycleConfigurationRuleEl {
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
                        terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>,
                    >,
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
                    ::Into<
                        terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>,
                    >,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
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

impl terrars::ToListMappable for S3BucketLifecycleConfigurationRuleEl {
    type O = terrars::BlockAssignable<S3BucketLifecycleConfigurationRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleConfigurationRuleEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLifecycleConfigurationRuleEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleEl {
        S3BucketLifecycleConfigurationRuleEl {
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

pub struct S3BucketLifecycleConfigurationRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLifecycleConfigurationRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLifecycleConfigurationRuleElRef {
        S3BucketLifecycleConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleConfigurationRuleElRef {
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
    ) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElExpirationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_version_expiration` after provisioning.\n"]
    pub fn noncurrent_version_expiration(
        &self,
    ) -> terrars::ListRef<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.noncurrent_version_expiration", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLifecycleConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketLifecycleConfigurationRuleEl>>,
}
