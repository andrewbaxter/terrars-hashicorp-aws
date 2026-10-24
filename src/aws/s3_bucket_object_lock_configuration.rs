use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketObjectLockConfigurationData {
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
    object_lock_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3BucketObjectLockConfigurationRuleEl>>,
    dynamic: S3BucketObjectLockConfigurationDynamic,
}

struct S3BucketObjectLockConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketObjectLockConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketObjectLockConfiguration(std::rc::Rc<S3BucketObjectLockConfiguration_>);

impl S3BucketObjectLockConfiguration {
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

    #[doc = "Set the field `object_lock_enabled`.\n"]
    pub fn set_object_lock_enabled(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().object_lock_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketObjectLockConfigurationRuleEl>>,
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

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketObjectLockConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketObjectLockConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketObjectLockConfiguration { }

impl terrars::ToListMappable for S3BucketObjectLockConfiguration {
    type O = terrars::ListRef<S3BucketObjectLockConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketObjectLockConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_object_lock_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketObjectLockConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketObjectLockConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketObjectLockConfiguration {
        let out = S3BucketObjectLockConfiguration(std::rc::Rc::new(S3BucketObjectLockConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketObjectLockConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                object_lock_enabled: core::default::Default::default(),
                token: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketObjectLockConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketObjectLockConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketObjectLockConfigurationRef {
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

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketObjectLockConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    years: std::option::Option<terrars::PrimField<f64>>,
}

impl S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `years`.\n"]
    pub fn set_years(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.years = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    type O = terrars::BlockAssignable<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationRuleElDefaultRetentionEl {}

impl BuildS3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
        S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
            days: core::default::Default::default(),
            mode: core::default::Default::default(),
            years: core::default::Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
        S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `years` after provisioning.\n"]
    pub fn years(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.years", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationRuleElDynamic {
    default_retention: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_retention: std::option::Option<std::vec::Vec<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
    dynamic: S3BucketObjectLockConfigurationRuleElDynamic,
}

impl S3BucketObjectLockConfigurationRuleEl {
    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketObjectLockConfigurationRuleEl {
    type O = terrars::BlockAssignable<S3BucketObjectLockConfigurationRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationRuleEl {}

impl BuildS3BucketObjectLockConfigurationRuleEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationRuleEl {
        S3BucketObjectLockConfigurationRuleEl {
            default_retention: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketObjectLockConfigurationRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketObjectLockConfigurationRuleElRef {
        S3BucketObjectLockConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> terrars::ListRef<S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketObjectLockConfigurationRuleEl>>,
}
