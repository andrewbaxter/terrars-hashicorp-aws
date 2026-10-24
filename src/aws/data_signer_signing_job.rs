use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSignerSigningJobData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    job_id: terrars::PrimField<std::string::String>,
}

struct DataSignerSigningJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSignerSigningJobData>,
}

#[derive(Clone)]
pub struct DataSignerSigningJob(std::rc::Rc<DataSignerSigningJob_>);

impl DataSignerSigningJob {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `completed_at` after provisioning.\n"]
    pub fn completed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_invoker` after provisioning.\n"]
    pub fn job_invoker(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_invoker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_owner` after provisioning.\n"]
    pub fn job_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_name` after provisioning.\n"]
    pub fn profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_version` after provisioning.\n"]
    pub fn profile_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<DataSignerSigningJobRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<DataSignerSigningJobSignedObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataSignerSigningJobSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

impl terrars::Referable for DataSignerSigningJob {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSignerSigningJob { }

impl terrars::ToListMappable for DataSignerSigningJob {
    type O = terrars::ListRef<DataSignerSigningJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSignerSigningJob_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_signer_signing_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSignerSigningJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub job_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSignerSigningJob {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSignerSigningJob {
        let out = DataSignerSigningJob(std::rc::Rc::new(DataSignerSigningJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSignerSigningJobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                job_id: self.job_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSignerSigningJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSignerSigningJobRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `completed_at` after provisioning.\n"]
    pub fn completed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_invoker` after provisioning.\n"]
    pub fn job_invoker(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_invoker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_owner` after provisioning.\n"]
    pub fn job_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_name` after provisioning.\n"]
    pub fn profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_version` after provisioning.\n"]
    pub fn profile_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<DataSignerSigningJobRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<DataSignerSigningJobSignedObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataSignerSigningJobSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobRevocationRecordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSignerSigningJobRevocationRecordEl {
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_at`.\n"]
    pub fn set_revoked_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revoked_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_by`.\n"]
    pub fn set_revoked_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revoked_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningJobRevocationRecordEl {
    type O = terrars::BlockAssignable<DataSignerSigningJobRevocationRecordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobRevocationRecordEl {}

impl BuildDataSignerSigningJobRevocationRecordEl {
    pub fn build(self) -> DataSignerSigningJobRevocationRecordEl {
        DataSignerSigningJobRevocationRecordEl {
            reason: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobRevocationRecordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobRevocationRecordElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningJobRevocationRecordElRef {
        DataSignerSigningJobRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobRevocationRecordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_at` after provisioning.\n"]
    pub fn revoked_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoked_at", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_by` after provisioning.\n"]
    pub fn revoked_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoked_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobSignedObjectElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSignerSigningJobSignedObjectElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningJobSignedObjectElS3El {
    type O = terrars::BlockAssignable<DataSignerSigningJobSignedObjectElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSignedObjectElS3El {}

impl BuildDataSignerSigningJobSignedObjectElS3El {
    pub fn build(self) -> DataSignerSigningJobSignedObjectElS3El {
        DataSignerSigningJobSignedObjectElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobSignedObjectElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobSignedObjectElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningJobSignedObjectElS3ElRef {
        DataSignerSigningJobSignedObjectElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSignedObjectElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobSignedObjectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::ListField<DataSignerSigningJobSignedObjectElS3El>>,
}

impl DataSignerSigningJobSignedObjectEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSignerSigningJobSignedObjectElS3El>>,
    ) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningJobSignedObjectEl {
    type O = terrars::BlockAssignable<DataSignerSigningJobSignedObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSignedObjectEl {}

impl BuildDataSignerSigningJobSignedObjectEl {
    pub fn build(self) -> DataSignerSigningJobSignedObjectEl {
        DataSignerSigningJobSignedObjectEl { s3: core::default::Default::default() }
    }
}

pub struct DataSignerSigningJobSignedObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobSignedObjectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningJobSignedObjectElRef {
        DataSignerSigningJobSignedObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSignedObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<DataSignerSigningJobSignedObjectElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobSourceElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSignerSigningJobSourceElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningJobSourceElS3El {
    type O = terrars::BlockAssignable<DataSignerSigningJobSourceElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSourceElS3El {}

impl BuildDataSignerSigningJobSourceElS3El {
    pub fn build(self) -> DataSignerSigningJobSourceElS3El {
        DataSignerSigningJobSourceElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobSourceElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobSourceElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningJobSourceElS3ElRef {
        DataSignerSigningJobSourceElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSourceElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::ListField<DataSignerSigningJobSourceElS3El>>,
}

impl DataSignerSigningJobSourceEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::ListField<DataSignerSigningJobSourceElS3El>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSignerSigningJobSourceEl {
    type O = terrars::BlockAssignable<DataSignerSigningJobSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSourceEl {}

impl BuildDataSignerSigningJobSourceEl {
    pub fn build(self) -> DataSignerSigningJobSourceEl {
        DataSignerSigningJobSourceEl { s3: core::default::Default::default() }
    }
}

pub struct DataSignerSigningJobSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSignerSigningJobSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSignerSigningJobSourceElRef {
        DataSignerSigningJobSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<DataSignerSigningJobSourceElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
