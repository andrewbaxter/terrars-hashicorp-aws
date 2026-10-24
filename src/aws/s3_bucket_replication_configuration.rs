use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketReplicationConfigurationData {
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
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleEl>>,
    dynamic: S3BucketReplicationConfigurationDynamic,
}

struct S3BucketReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketReplicationConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketReplicationConfiguration(std::rc::Rc<S3BucketReplicationConfiguration_>);

impl S3BucketReplicationConfiguration {
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

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleEl>>,
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

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketReplicationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketReplicationConfiguration { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration {
    type O = terrars::ListRef<S3BucketReplicationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketReplicationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketReplicationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketReplicationConfiguration {
        let out = S3BucketReplicationConfiguration(std::rc::Rc::new(S3BucketReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketReplicationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                id: core::default::Default::default(),
                role: self.role,
                token: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketReplicationConfigurationRef {
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

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl {
        S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl { status: self.status }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
        S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    owner: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl {
        S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl { owner: self.owner }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    replica_kms_key_id: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    #[doc = ""]
    pub replica_kms_key_id: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
        S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl {
            replica_kms_key_id: self.replica_kms_key_id,
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replica_kms_key_id` after provisioning.\n"]
    pub fn replica_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    minutes: terrars::PrimField<f64>,
}

impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    #[doc = ""]
    pub minutes: terrars::PrimField<f64>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl { minutes: self.minutes }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElDynamic {
    event_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_threshold: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElMetricsElDynamic,
}

impl S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    #[doc = "Set the field `event_threshold`.\n"]
    pub fn set_event_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_threshold = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsEl {
            status: self.status,
            event_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
        S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `event_threshold` after provisioning.\n"]
    pub fn event_threshold(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElMetricsElEventThresholdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    minutes: terrars::PrimField<f64>,
}

impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    #[doc = ""]
    pub minutes: terrars::PrimField<f64>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl { minutes: self.minutes }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElDynamic {
    time: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElDynamic,
}

impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl {
            status: self.status,
            time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
        S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDestinationElDynamic {
    access_control_translation: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl>>,
    metrics: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    replication_time: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_translation: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_time: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDestinationElDynamic,
}

impl S3BucketReplicationConfigurationRuleElDestinationEl {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_translation`.\n"]
    pub fn set_access_control_translation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_translation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_translation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElMetricsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_time`.\n"]
    pub fn set_replication_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replication_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElDestinationEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElDestinationEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElDestinationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElDestinationEl {
        S3BucketReplicationConfigurationRuleElDestinationEl {
            account: core::default::Default::default(),
            bucket: self.bucket,
            storage_class: core::default::Default::default(),
            access_control_translation: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            metrics: core::default::Default::default(),
            replication_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElDestinationElRef {
        S3BucketReplicationConfigurationRuleElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_translation` after provisioning.\n"]
    pub fn access_control_translation(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElAccessControlTranslationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_translation", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElMetricsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_time` after provisioning.\n"]
    pub fn replication_time(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElReplicationTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_time", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl {
        S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl { status: self.status }
    }
}

pub struct S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
        S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElFilterElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3BucketReplicationConfigurationRuleElFilterElAndEl {
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

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElFilterElAndEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElFilterElAndEl {}

impl BuildS3BucketReplicationConfigurationRuleElFilterElAndEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterElAndEl {
        S3BucketReplicationConfigurationRuleElFilterElAndEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElFilterElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElFilterElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElFilterElAndElRef {
        S3BucketReplicationConfigurationRuleElFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElFilterElAndElRef {
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
pub struct S3BucketReplicationConfigurationRuleElFilterElTagEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElFilterElTagEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElFilterElTagEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElFilterElTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElFilterElTagEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterElTagEl {
        S3BucketReplicationConfigurationRuleElFilterElTagEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElFilterElTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElFilterElTagElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElFilterElTagElRef {
        S3BucketReplicationConfigurationRuleElFilterElTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElFilterElTagElRef {
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
struct S3BucketReplicationConfigurationRuleElFilterElDynamic {
    and: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
    tag: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElFilterElDynamic,
}

impl S3BucketReplicationConfigurationRuleElFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElAndEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterElTagEl>>,
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

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElFilterEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElFilterEl {}

impl BuildS3BucketReplicationConfigurationRuleElFilterEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElFilterEl {
        S3BucketReplicationConfigurationRuleElFilterEl {
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElFilterElRef {
        S3BucketReplicationConfigurationRuleElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElFilterElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElFilterElTagElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl { }

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    type O =
        terrars
        ::BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl { status: self.status }
    }
}

pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl { }

impl terrars
::ToListMappable for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    type O =
        terrars
        ::BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
            status: self.status,
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElDynamic {
    replica_modifications: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl>,
    >,
    sse_kms_encrypted_objects: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_modifications: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_kms_encrypted_objects: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElDynamic,
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    #[doc = "Set the field `replica_modifications`.\n"]
    pub fn set_replica_modifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replica_modifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_modifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_kms_encrypted_objects`.\n"]
    pub fn set_sse_kms_encrypted_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_kms_encrypted_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms_encrypted_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {}

impl BuildS3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl {
            replica_modifications: core::default::Default::default(),
            sse_kms_encrypted_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
        S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replica_modifications` after provisioning.\n"]
    pub fn replica_modifications(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElReplicaModificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_modifications", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_kms_encrypted_objects` after provisioning.\n"]
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_kms_encrypted_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationRuleElDynamic {
    delete_marker_replication: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    destination: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElDestinationEl>>,
    existing_object_replication: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>>,
    filter: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElFilterEl>>,
    source_selection_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_marker_replication: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_object_replication: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_selection_criteria: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
    dynamic: S3BucketReplicationConfigurationRuleElDynamic,
}

impl S3BucketReplicationConfigurationRuleEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_marker_replication`.\n"]
    pub fn set_delete_marker_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delete_marker_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_marker_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElDestinationEl>>,
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

    #[doc = "Set the field `existing_object_replication`.\n"]
    pub fn set_existing_object_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElExistingObjectReplicationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_object_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_object_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElFilterEl>>,
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

    #[doc = "Set the field `source_selection_criteria`.\n"]
    pub fn set_source_selection_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_selection_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_selection_criteria = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfigurationRuleEl {
    type O = terrars::BlockAssignable<S3BucketReplicationConfigurationRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationRuleEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfigurationRuleEl {
    pub fn build(self) -> S3BucketReplicationConfigurationRuleEl {
        S3BucketReplicationConfigurationRuleEl {
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            priority: core::default::Default::default(),
            status: self.status,
            delete_marker_replication: core::default::Default::default(),
            destination: core::default::Default::default(),
            existing_object_replication: core::default::Default::default(),
            filter: core::default::Default::default(),
            source_selection_criteria: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketReplicationConfigurationRuleElRef {
        S3BucketReplicationConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationRuleElRef {
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

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_marker_replication` after provisioning.\n"]
    pub fn delete_marker_replication(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDeleteMarkerReplicationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_marker_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `existing_object_replication` after provisioning.\n"]
    pub fn existing_object_replication(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElExistingObjectReplicationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_object_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `source_selection_criteria` after provisioning.\n"]
    pub fn source_selection_criteria(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfigurationRuleElSourceSelectionCriteriaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_selection_criteria", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfigurationRuleEl>>,
}
