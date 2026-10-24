use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketServerSideEncryptionConfigurationData {
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
    rule: std::option::Option<std::vec::Vec<S3BucketServerSideEncryptionConfigurationRuleEl>>,
    dynamic: S3BucketServerSideEncryptionConfigurationDynamic,
}

struct S3BucketServerSideEncryptionConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketServerSideEncryptionConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketServerSideEncryptionConfiguration(std::rc::Rc<S3BucketServerSideEncryptionConfiguration_>);

impl S3BucketServerSideEncryptionConfiguration {
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleEl>>,
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
}

impl terrars::Referable for S3BucketServerSideEncryptionConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketServerSideEncryptionConfiguration { }

impl terrars::ToListMappable for S3BucketServerSideEncryptionConfiguration {
    type O = terrars::ListRef<S3BucketServerSideEncryptionConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketServerSideEncryptionConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_server_side_encryption_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketServerSideEncryptionConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketServerSideEncryptionConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketServerSideEncryptionConfiguration {
        let out =
            S3BucketServerSideEncryptionConfiguration(std::rc::Rc::new(S3BucketServerSideEncryptionConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(S3BucketServerSideEncryptionConfigurationData {
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

pub struct S3BucketServerSideEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketServerSideEncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRef {
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
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_master_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    sse_algorithm: terrars::PrimField<std::string::String>,
}

impl S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = "Set the field `kms_master_key_id`.\n"]
    pub fn set_kms_master_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_master_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    type O =
        terrars
        ::BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = ""]
    pub sse_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
        S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
            kms_master_key_id: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
        S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_master_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationRuleElDynamic {
    apply_server_side_encryption_by_default: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_key_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apply_server_side_encryption_by_default: std
    ::option
    ::Option<std::vec::Vec<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>>,
    dynamic: S3BucketServerSideEncryptionConfigurationRuleElDynamic,
}

impl S3BucketServerSideEncryptionConfigurationRuleEl {
    #[doc = "Set the field `bucket_key_enabled`.\n"]
    pub fn set_bucket_key_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bucket_key_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apply_server_side_encryption_by_default`.\n"]
    pub fn set_apply_server_side_encryption_by_default(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apply_server_side_encryption_by_default = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apply_server_side_encryption_by_default = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketServerSideEncryptionConfigurationRuleEl {
    type O = terrars::BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationRuleEl {}

impl BuildS3BucketServerSideEncryptionConfigurationRuleEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationRuleEl {
        S3BucketServerSideEncryptionConfigurationRuleEl {
            bucket_key_enabled: core::default::Default::default(),
            apply_server_side_encryption_by_default: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketServerSideEncryptionConfigurationRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketServerSideEncryptionConfigurationRuleElRef {
        S3BucketServerSideEncryptionConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_key_enabled` after provisioning.\n"]
    pub fn bucket_key_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `apply_server_side_encryption_by_default` after provisioning.\n"]
    pub fn apply_server_side_encryption_by_default(
        &self,
    ) -> terrars::ListRef<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.apply_server_side_encryption_by_default", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketServerSideEncryptionConfigurationRuleEl>>,
}
