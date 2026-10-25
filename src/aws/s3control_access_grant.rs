#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlAccessGrantData_ {
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
    access_grants_location_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    permission: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_prefix_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_grants_location_configuration: std
    ::option
    ::Option<std::vec::Vec<S3controlAccessGrant_AccessGrantsLocationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee: std::option::Option<std::vec::Vec<S3controlAccessGrant_Grantee>>,
    dynamic: S3controlAccessGrantDynamic,
}

struct S3controlAccessGrant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlAccessGrantData_>,
}

#[derive(Clone)]
pub struct S3controlAccessGrant(std::rc::Rc<S3controlAccessGrant_>);

impl S3controlAccessGrant {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix_type`.\n"]
    pub fn set_s3_prefix_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().s3_prefix_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_grants_location_configuration`.\n"]
    pub fn set_access_grants_location_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlAccessGrant_AccessGrantsLocationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_grants_location_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_grants_location_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlAccessGrant_Grantee>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grantee = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grantee = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_grant_arn` after provisioning.\n"]
    pub fn access_grant_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grant_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grant_id` after provisioning.\n"]
    pub fn access_grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_id` after provisioning.\n"]
    pub fn access_grants_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grants_location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_scope` after provisioning.\n"]
    pub fn grant_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix_type` after provisioning.\n"]
    pub fn s3_prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_configuration` after provisioning.\n"]
    pub fn access_grants_location_configuration(
        &self,
    ) -> terrars::ListRef<S3controlAccessGrant_AccessGrantsLocationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.access_grants_location_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::ListRef<S3controlAccessGrant_GranteeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantee", self.extract_ref()))
    }
}

impl terrars::Referable for S3controlAccessGrant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlAccessGrant { }

impl terrars::ToListMappable for S3controlAccessGrant {
    type O = terrars::ListRef<S3controlAccessGrantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlAccessGrant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_access_grant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlAccessGrant {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub access_grants_location_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
}

impl BuildS3controlAccessGrant {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlAccessGrant {
        let out = S3controlAccessGrant(std::rc::Rc::new(S3controlAccessGrant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlAccessGrantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_grants_location_id: self.access_grants_location_id,
                account_id: core::default::Default::default(),
                permission: self.permission,
                region: core::default::Default::default(),
                s3_prefix_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                access_grants_location_configuration: core::default::Default::default(),
                grantee: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlAccessGrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlAccessGrantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlAccessGrantRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_grant_arn` after provisioning.\n"]
    pub fn access_grant_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grant_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grant_id` after provisioning.\n"]
    pub fn access_grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_id` after provisioning.\n"]
    pub fn access_grants_location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_grants_location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_scope` after provisioning.\n"]
    pub fn grant_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix_type` after provisioning.\n"]
    pub fn s3_prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_configuration` after provisioning.\n"]
    pub fn access_grants_location_configuration(
        &self,
    ) -> terrars::ListRef<S3controlAccessGrant_AccessGrantsLocationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.access_grants_location_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::ListRef<S3controlAccessGrant_GranteeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantee", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3controlAccessGrantDynamic {
    access_grants_location_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlAccessGrant_AccessGrantsLocationConfiguration>>,
    grantee: std::option::Option<terrars::DynamicBlock<S3controlAccessGrant_Grantee>>,
}

#[derive(Serialize)]
pub struct S3controlAccessGrant_AccessGrantsLocationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_sub_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3controlAccessGrant_AccessGrantsLocationConfiguration {
    #[doc = "Set the field `s3_sub_prefix`.\n"]
    pub fn set_s3_sub_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_sub_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlAccessGrant_AccessGrantsLocationConfiguration {
    type O = terrars::BlockAssignable<S3controlAccessGrant_AccessGrantsLocationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlAccessGrant_AccessGrantsLocationConfiguration {}

impl BuildS3controlAccessGrant_AccessGrantsLocationConfiguration {
    pub fn build(self) -> S3controlAccessGrant_AccessGrantsLocationConfiguration {
        S3controlAccessGrant_AccessGrantsLocationConfiguration { s3_sub_prefix: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct S3controlAccessGrant_AccessGrantsLocationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlAccessGrant_AccessGrantsLocationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlAccessGrant_AccessGrantsLocationConfigurationRef {
        S3controlAccessGrant_AccessGrantsLocationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlAccessGrant_AccessGrantsLocationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_sub_prefix` after provisioning.\n"]
    pub fn s3_sub_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_sub_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlAccessGrant_Grantee {
    grantee_identifier: terrars::PrimField<std::string::String>,
    grantee_type: terrars::PrimField<std::string::String>,
}

impl S3controlAccessGrant_Grantee { }

impl terrars::ToListMappable for S3controlAccessGrant_Grantee {
    type O = terrars::BlockAssignable<S3controlAccessGrant_Grantee>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlAccessGrant_Grantee {
    #[doc = ""]
    pub grantee_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub grantee_type: terrars::PrimField<std::string::String>,
}

impl BuildS3controlAccessGrant_Grantee {
    pub fn build(self) -> S3controlAccessGrant_Grantee {
        S3controlAccessGrant_Grantee {
            grantee_identifier: self.grantee_identifier,
            grantee_type: self.grantee_type,
        }
    }
}

#[allow(dead_code)]
pub struct S3controlAccessGrant_GranteeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlAccessGrant_GranteeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3controlAccessGrant_GranteeRef {
        S3controlAccessGrant_GranteeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlAccessGrant_GranteeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grantee_identifier` after provisioning.\n"]
    pub fn grantee_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}
