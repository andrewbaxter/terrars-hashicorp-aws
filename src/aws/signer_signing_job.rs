use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SignerSigningJobData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_signing_job_failure: std::option::Option<terrars::PrimField<bool>>,
    profile_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<SignerSigningJobDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<SignerSigningJobSourceEl>>,
    dynamic: SignerSigningJobDynamic,
}

struct SignerSigningJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SignerSigningJobData>,
}

#[derive(Clone)]
pub struct SignerSigningJob(std::rc::Rc<SignerSigningJob_>);

impl SignerSigningJob {
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

    #[doc = "Set the field `ignore_signing_job_failure`.\n"]
    pub fn set_ignore_signing_job_failure(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_signing_job_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJobDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJobSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `ignore_signing_job_failure` after provisioning.\n"]
    pub fn ignore_signing_job_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_signing_job_failure", self.extract_ref()))
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
    pub fn revocation_record(&self) -> terrars::ListRef<SignerSigningJobRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<SignerSigningJobSignedObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<SignerSigningJobDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SignerSigningJobSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for SignerSigningJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SignerSigningJob { }

impl terrars::ToListMappable for SignerSigningJob {
    type O = terrars::ListRef<SignerSigningJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SignerSigningJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_signer_signing_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSignerSigningJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub profile_name: terrars::PrimField<std::string::String>,
}

impl BuildSignerSigningJob {
    pub fn build(self, stack: &mut terrars::Stack) -> SignerSigningJob {
        let out = SignerSigningJob(std::rc::Rc::new(SignerSigningJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SignerSigningJobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_signing_job_failure: core::default::Default::default(),
                profile_name: self.profile_name,
                destination: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SignerSigningJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SignerSigningJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `ignore_signing_job_failure` after provisioning.\n"]
    pub fn ignore_signing_job_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_signing_job_failure", self.extract_ref()))
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
    pub fn revocation_record(&self) -> terrars::ListRef<SignerSigningJobRevocationRecordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<SignerSigningJobSignedObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<SignerSigningJobDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SignerSigningJobSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobRevocationRecordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJobRevocationRecordEl {
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

impl terrars::ToListMappable for SignerSigningJobRevocationRecordEl {
    type O = terrars::BlockAssignable<SignerSigningJobRevocationRecordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobRevocationRecordEl {}

impl BuildSignerSigningJobRevocationRecordEl {
    pub fn build(self) -> SignerSigningJobRevocationRecordEl {
        SignerSigningJobRevocationRecordEl {
            reason: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobRevocationRecordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobRevocationRecordElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobRevocationRecordElRef {
        SignerSigningJobRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobRevocationRecordElRef {
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
pub struct SignerSigningJobSignedObjectElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJobSignedObjectElS3El {
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

impl terrars::ToListMappable for SignerSigningJobSignedObjectElS3El {
    type O = terrars::BlockAssignable<SignerSigningJobSignedObjectElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSignedObjectElS3El {}

impl BuildSignerSigningJobSignedObjectElS3El {
    pub fn build(self) -> SignerSigningJobSignedObjectElS3El {
        SignerSigningJobSignedObjectElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobSignedObjectElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobSignedObjectElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobSignedObjectElS3ElRef {
        SignerSigningJobSignedObjectElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSignedObjectElS3ElRef {
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
pub struct SignerSigningJobSignedObjectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::ListField<SignerSigningJobSignedObjectElS3El>>,
}

impl SignerSigningJobSignedObjectEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::ListField<SignerSigningJobSignedObjectElS3El>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SignerSigningJobSignedObjectEl {
    type O = terrars::BlockAssignable<SignerSigningJobSignedObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSignedObjectEl {}

impl BuildSignerSigningJobSignedObjectEl {
    pub fn build(self) -> SignerSigningJobSignedObjectEl {
        SignerSigningJobSignedObjectEl { s3: core::default::Default::default() }
    }
}

pub struct SignerSigningJobSignedObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobSignedObjectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobSignedObjectElRef {
        SignerSigningJobSignedObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSignedObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJobSignedObjectElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobDestinationElS3El {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJobDestinationElS3El {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SignerSigningJobDestinationElS3El {
    type O = terrars::BlockAssignable<SignerSigningJobDestinationElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobDestinationElS3El {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildSignerSigningJobDestinationElS3El {
    pub fn build(self) -> SignerSigningJobDestinationElS3El {
        SignerSigningJobDestinationElS3El {
            bucket: self.bucket,
            prefix: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobDestinationElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobDestinationElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobDestinationElS3ElRef {
        SignerSigningJobDestinationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobDestinationElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobDestinationElDynamic {
    s3: std::option::Option<terrars::DynamicBlock<SignerSigningJobDestinationElS3El>>,
}

#[derive(Serialize)]
pub struct SignerSigningJobDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<SignerSigningJobDestinationElS3El>>,
    dynamic: SignerSigningJobDestinationElDynamic,
}

impl SignerSigningJobDestinationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJobDestinationElS3El>>,
    ) -> Self {
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

impl terrars::ToListMappable for SignerSigningJobDestinationEl {
    type O = terrars::BlockAssignable<SignerSigningJobDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobDestinationEl {}

impl BuildSignerSigningJobDestinationEl {
    pub fn build(self) -> SignerSigningJobDestinationEl {
        SignerSigningJobDestinationEl {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobDestinationElRef {
        SignerSigningJobDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJobDestinationElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobSourceElS3El {
    bucket: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl SignerSigningJobSourceElS3El { }

impl terrars::ToListMappable for SignerSigningJobSourceElS3El {
    type O = terrars::BlockAssignable<SignerSigningJobSourceElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSourceElS3El {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildSignerSigningJobSourceElS3El {
    pub fn build(self) -> SignerSigningJobSourceElS3El {
        SignerSigningJobSourceElS3El {
            bucket: self.bucket,
            key: self.key,
            version: self.version,
        }
    }
}

pub struct SignerSigningJobSourceElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobSourceElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobSourceElS3ElRef {
        SignerSigningJobSourceElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSourceElS3ElRef {
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

#[derive(Serialize, Default)]
struct SignerSigningJobSourceElDynamic {
    s3: std::option::Option<terrars::DynamicBlock<SignerSigningJobSourceElS3El>>,
}

#[derive(Serialize)]
pub struct SignerSigningJobSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<SignerSigningJobSourceElS3El>>,
    dynamic: SignerSigningJobSourceElDynamic,
}

impl SignerSigningJobSourceEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJobSourceElS3El>>) -> Self {
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

impl terrars::ToListMappable for SignerSigningJobSourceEl {
    type O = terrars::BlockAssignable<SignerSigningJobSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSourceEl {}

impl BuildSignerSigningJobSourceEl {
    pub fn build(self) -> SignerSigningJobSourceEl {
        SignerSigningJobSourceEl {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJobSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJobSourceElRef {
        SignerSigningJobSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJobSourceElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobDynamic {
    destination: std::option::Option<terrars::DynamicBlock<SignerSigningJobDestinationEl>>,
    source: std::option::Option<terrars::DynamicBlock<SignerSigningJobSourceEl>>,
}
