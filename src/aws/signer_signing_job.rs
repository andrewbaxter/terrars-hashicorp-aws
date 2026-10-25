#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SignerSigningJobData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<SignerSigningJob_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<SignerSigningJob_Source>>,
    dynamic: SignerSigningJobDynamic,
}

struct SignerSigningJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SignerSigningJobData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJob_Destination>>,
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
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJob_Source>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<SignerSigningJob_RevocationRecordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<SignerSigningJob_SignedObjectRef> {
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
    pub fn destination(&self) -> terrars::ListRef<SignerSigningJob_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SignerSigningJob_SourceRef> {
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
            data: std::cell::RefCell::new(SignerSigningJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_signing_job_failure: core::default::Default::default(),
                profile_name: self.profile_name,
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> terrars::ListRef<SignerSigningJob_RevocationRecordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> terrars::ListRef<SignerSigningJob_SignedObjectRef> {
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
    pub fn destination(&self) -> terrars::ListRef<SignerSigningJob_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SignerSigningJob_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobDynamic {
    destination: std::option::Option<terrars::DynamicBlock<SignerSigningJob_Destination>>,
    source: std::option::Option<terrars::DynamicBlock<SignerSigningJob_Source>>,
}

#[derive(Serialize)]
pub struct SignerSigningJob_RevocationRecord {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJob_RevocationRecord {
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

impl terrars::ToListMappable for SignerSigningJob_RevocationRecord {
    type O = terrars::BlockAssignable<SignerSigningJob_RevocationRecord>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_RevocationRecord {}

impl BuildSignerSigningJob_RevocationRecord {
    pub fn build(self) -> SignerSigningJob_RevocationRecord {
        SignerSigningJob_RevocationRecord {
            reason: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_RevocationRecordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_RevocationRecordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_RevocationRecordRef {
        SignerSigningJob_RevocationRecordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_RevocationRecordRef {
    #[allow(dead_code)]
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
pub struct SignerSigningJob_SignedObject_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJob_SignedObject_S3 {
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

impl terrars::ToListMappable for SignerSigningJob_SignedObject_S3 {
    type O = terrars::BlockAssignable<SignerSigningJob_SignedObject_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_SignedObject_S3 {}

impl BuildSignerSigningJob_SignedObject_S3 {
    pub fn build(self) -> SignerSigningJob_SignedObject_S3 {
        SignerSigningJob_SignedObject_S3 {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_SignedObject_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_SignedObject_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_SignedObject_S3Ref {
        SignerSigningJob_SignedObject_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_SignedObject_S3Ref {
    #[allow(dead_code)]
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
pub struct SignerSigningJob_SignedObject {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::ListField<SignerSigningJob_SignedObject_S3>>,
}

impl SignerSigningJob_SignedObject {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::ListField<SignerSigningJob_SignedObject_S3>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SignerSigningJob_SignedObject {
    type O = terrars::BlockAssignable<SignerSigningJob_SignedObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_SignedObject {}

impl BuildSignerSigningJob_SignedObject {
    pub fn build(self) -> SignerSigningJob_SignedObject {
        SignerSigningJob_SignedObject { s3: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_SignedObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_SignedObjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_SignedObjectRef {
        SignerSigningJob_SignedObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_SignedObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJob_SignedObject_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJob_Destination_S3 {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SignerSigningJob_Destination_S3 {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SignerSigningJob_Destination_S3 {
    type O = terrars::BlockAssignable<SignerSigningJob_Destination_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_Destination_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildSignerSigningJob_Destination_S3 {
    pub fn build(self) -> SignerSigningJob_Destination_S3 {
        SignerSigningJob_Destination_S3 {
            bucket: self.bucket,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_Destination_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_Destination_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_Destination_S3Ref {
        SignerSigningJob_Destination_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_Destination_S3Ref {
    #[allow(dead_code)]
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
struct SignerSigningJob_DestinationDynamic {
    s3: std::option::Option<terrars::DynamicBlock<SignerSigningJob_Destination_S3>>,
}

#[derive(Serialize)]
pub struct SignerSigningJob_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<SignerSigningJob_Destination_S3>>,
    dynamic: SignerSigningJob_DestinationDynamic,
}

impl SignerSigningJob_Destination {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJob_Destination_S3>>,
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

impl terrars::ToListMappable for SignerSigningJob_Destination {
    type O = terrars::BlockAssignable<SignerSigningJob_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_Destination {}

impl BuildSignerSigningJob_Destination {
    pub fn build(self) -> SignerSigningJob_Destination {
        SignerSigningJob_Destination {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_DestinationRef {
        SignerSigningJob_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJob_Destination_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJob_Source_S3 {
    bucket: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl SignerSigningJob_Source_S3 { }

impl terrars::ToListMappable for SignerSigningJob_Source_S3 {
    type O = terrars::BlockAssignable<SignerSigningJob_Source_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_Source_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildSignerSigningJob_Source_S3 {
    pub fn build(self) -> SignerSigningJob_Source_S3 {
        SignerSigningJob_Source_S3 {
            bucket: self.bucket,
            key: self.key,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_Source_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_Source_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_Source_S3Ref {
        SignerSigningJob_Source_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_Source_S3Ref {
    #[allow(dead_code)]
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
struct SignerSigningJob_SourceDynamic {
    s3: std::option::Option<terrars::DynamicBlock<SignerSigningJob_Source_S3>>,
}

#[derive(Serialize)]
pub struct SignerSigningJob_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<SignerSigningJob_Source_S3>>,
    dynamic: SignerSigningJob_SourceDynamic,
}

impl SignerSigningJob_Source {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::BlockAssignable<SignerSigningJob_Source_S3>>) -> Self {
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

impl terrars::ToListMappable for SignerSigningJob_Source {
    type O = terrars::BlockAssignable<SignerSigningJob_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJob_Source {}

impl BuildSignerSigningJob_Source {
    pub fn build(self) -> SignerSigningJob_Source {
        SignerSigningJob_Source {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SignerSigningJob_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SignerSigningJob_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SignerSigningJob_SourceRef {
        SignerSigningJob_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJob_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<SignerSigningJob_Source_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
