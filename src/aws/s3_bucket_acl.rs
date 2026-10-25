#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketAclData_ {
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
    acl: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_policy: std::option::Option<std::vec::Vec<S3BucketAcl_AccessControlPolicy>>,
    dynamic: S3BucketAclDynamic,
}

struct S3BucketAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketAclData_>,
}

#[derive(Clone)]
pub struct S3BucketAcl(std::rc::Rc<S3BucketAcl_>);

impl S3BucketAcl {
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

    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().acl = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_policy`.\n"]
    pub fn set_access_control_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_control_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_control_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `access_control_policy` after provisioning.\n"]
    pub fn access_control_policy(&self) -> terrars::ListRef<S3BucketAcl_AccessControlPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_policy", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketAcl { }

impl terrars::ToListMappable for S3BucketAcl {
    type O = terrars::ListRef<S3BucketAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketAcl {
        let out = S3BucketAcl(std::rc::Rc::new(S3BucketAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                access_control_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketAclRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `access_control_policy` after provisioning.\n"]
    pub fn access_control_policy(&self) -> terrars::ListRef<S3BucketAcl_AccessControlPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAclDynamic {
    access_control_policy: std::option::Option<terrars::DynamicBlock<S3BucketAcl_AccessControlPolicy>>,
}

#[derive(Serialize)]
pub struct S3BucketAcl_AccessControlPolicy_Grant_Grantee {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketAcl_AccessControlPolicy_Grant_Grantee {
    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketAcl_AccessControlPolicy_Grant_Grantee {
    type O = terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Grant_Grantee>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAcl_AccessControlPolicy_Grant_Grantee {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAcl_AccessControlPolicy_Grant_Grantee {
    pub fn build(self) -> S3BucketAcl_AccessControlPolicy_Grant_Grantee {
        S3BucketAcl_AccessControlPolicy_Grant_Grantee {
            email_address: core::default::Default::default(),
            id: core::default::Default::default(),
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketAcl_AccessControlPolicy_Grant_GranteeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAcl_AccessControlPolicy_Grant_GranteeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketAcl_AccessControlPolicy_Grant_GranteeRef {
        S3BucketAcl_AccessControlPolicy_Grant_GranteeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAcl_AccessControlPolicy_Grant_GranteeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAcl_AccessControlPolicy_GrantDynamic {
    grantee: std::option::Option<terrars::DynamicBlock<S3BucketAcl_AccessControlPolicy_Grant_Grantee>>,
}

#[derive(Serialize)]
pub struct S3BucketAcl_AccessControlPolicy_Grant {
    permission: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee: std::option::Option<std::vec::Vec<S3BucketAcl_AccessControlPolicy_Grant_Grantee>>,
    dynamic: S3BucketAcl_AccessControlPolicy_GrantDynamic,
}

impl S3BucketAcl_AccessControlPolicy_Grant {
    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Grant_Grantee>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grantee = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grantee = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketAcl_AccessControlPolicy_Grant {
    type O = terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Grant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAcl_AccessControlPolicy_Grant {
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAcl_AccessControlPolicy_Grant {
    pub fn build(self) -> S3BucketAcl_AccessControlPolicy_Grant {
        S3BucketAcl_AccessControlPolicy_Grant {
            permission: self.permission,
            grantee: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketAcl_AccessControlPolicy_GrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAcl_AccessControlPolicy_GrantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketAcl_AccessControlPolicy_GrantRef {
        S3BucketAcl_AccessControlPolicy_GrantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAcl_AccessControlPolicy_GrantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::ListRef<S3BucketAcl_AccessControlPolicy_Grant_GranteeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketAcl_AccessControlPolicy_Owner {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
}

impl S3BucketAcl_AccessControlPolicy_Owner {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketAcl_AccessControlPolicy_Owner {
    type O = terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Owner>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAcl_AccessControlPolicy_Owner {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketAcl_AccessControlPolicy_Owner {
    pub fn build(self) -> S3BucketAcl_AccessControlPolicy_Owner {
        S3BucketAcl_AccessControlPolicy_Owner {
            display_name: core::default::Default::default(),
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketAcl_AccessControlPolicy_OwnerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAcl_AccessControlPolicy_OwnerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketAcl_AccessControlPolicy_OwnerRef {
        S3BucketAcl_AccessControlPolicy_OwnerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAcl_AccessControlPolicy_OwnerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAcl_AccessControlPolicyDynamic {
    grant: std::option::Option<terrars::DynamicBlock<S3BucketAcl_AccessControlPolicy_Grant>>,
    owner: std::option::Option<terrars::DynamicBlock<S3BucketAcl_AccessControlPolicy_Owner>>,
}

#[derive(Serialize)]
pub struct S3BucketAcl_AccessControlPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant: std::option::Option<std::vec::Vec<S3BucketAcl_AccessControlPolicy_Grant>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<std::vec::Vec<S3BucketAcl_AccessControlPolicy_Owner>>,
    dynamic: S3BucketAcl_AccessControlPolicyDynamic,
}

impl S3BucketAcl_AccessControlPolicy {
    #[doc = "Set the field `grant`.\n"]
    pub fn set_grant(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Grant>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grant = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy_Owner>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.owner = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.owner = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketAcl_AccessControlPolicy {
    type O = terrars::BlockAssignable<S3BucketAcl_AccessControlPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAcl_AccessControlPolicy {}

impl BuildS3BucketAcl_AccessControlPolicy {
    pub fn build(self) -> S3BucketAcl_AccessControlPolicy {
        S3BucketAcl_AccessControlPolicy {
            grant: core::default::Default::default(),
            owner: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketAcl_AccessControlPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketAcl_AccessControlPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketAcl_AccessControlPolicyRef {
        S3BucketAcl_AccessControlPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAcl_AccessControlPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::ListRef<S3BucketAcl_AccessControlPolicy_OwnerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}
