use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3AccessPointData {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access_block_configuration: std::option::Option<std::vec::Vec<S3AccessPointPublicAccessBlockConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_configuration: std::option::Option<std::vec::Vec<S3AccessPointVpcConfigurationEl>>,
    dynamic: S3AccessPointDynamic,
}

struct S3AccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3AccessPointData>,
}

#[derive(Clone)]
pub struct S3AccessPoint(std::rc::Rc<S3AccessPoint_>);

impl S3AccessPoint {
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

    #[doc = "Set the field `bucket_account_id`.\n"]
    pub fn set_bucket_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access_block_configuration`.\n"]
    pub fn set_public_access_block_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3AccessPointPublicAccessBlockConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_access_block_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_access_block_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3AccessPointVpcConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_public_access_policy` after provisioning.\n"]
    pub fn has_public_access_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_public_access_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(
        &self,
    ) -> terrars::ListRef<S3AccessPointPublicAccessBlockConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<S3AccessPointVpcConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for S3AccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3AccessPoint { }

impl terrars::ToListMappable for S3AccessPoint {
    type O = terrars::ListRef<S3AccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3AccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3AccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3AccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> S3AccessPoint {
        let out = S3AccessPoint(std::rc::Rc::new(S3AccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3AccessPointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                bucket: self.bucket,
                bucket_account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                policy: core::default::Default::default(),
                public_access_block_configuration: core::default::Default::default(),
                vpc_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3AccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3AccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3AccessPointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_public_access_policy` after provisioning.\n"]
    pub fn has_public_access_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_public_access_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(
        &self,
    ) -> terrars::ListRef<S3AccessPointPublicAccessBlockConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<S3AccessPointVpcConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3AccessPointPublicAccessBlockConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrict_public_buckets: std::option::Option<terrars::PrimField<bool>>,
}

impl S3AccessPointPublicAccessBlockConfigurationEl {
    #[doc = "Set the field `block_public_acls`.\n"]
    pub fn set_block_public_acls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_public_acls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_public_policy`.\n"]
    pub fn set_block_public_policy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_public_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_public_acls`.\n"]
    pub fn set_ignore_public_acls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_public_acls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restrict_public_buckets`.\n"]
    pub fn set_restrict_public_buckets(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.restrict_public_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3AccessPointPublicAccessBlockConfigurationEl {
    type O = terrars::BlockAssignable<S3AccessPointPublicAccessBlockConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3AccessPointPublicAccessBlockConfigurationEl {}

impl BuildS3AccessPointPublicAccessBlockConfigurationEl {
    pub fn build(self) -> S3AccessPointPublicAccessBlockConfigurationEl {
        S3AccessPointPublicAccessBlockConfigurationEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

pub struct S3AccessPointPublicAccessBlockConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3AccessPointPublicAccessBlockConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3AccessPointPublicAccessBlockConfigurationElRef {
        S3AccessPointPublicAccessBlockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3AccessPointPublicAccessBlockConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_acls` after provisioning.\n"]
    pub fn block_public_acls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `block_public_policy` after provisioning.\n"]
    pub fn block_public_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_public_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_public_acls` after provisioning.\n"]
    pub fn ignore_public_acls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `restrict_public_buckets` after provisioning.\n"]
    pub fn restrict_public_buckets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrict_public_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct S3AccessPointVpcConfigurationEl {
    vpc_id: terrars::PrimField<std::string::String>,
}

impl S3AccessPointVpcConfigurationEl { }

impl terrars::ToListMappable for S3AccessPointVpcConfigurationEl {
    type O = terrars::BlockAssignable<S3AccessPointVpcConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3AccessPointVpcConfigurationEl {
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildS3AccessPointVpcConfigurationEl {
    pub fn build(self) -> S3AccessPointVpcConfigurationEl {
        S3AccessPointVpcConfigurationEl { vpc_id: self.vpc_id }
    }
}

pub struct S3AccessPointVpcConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3AccessPointVpcConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3AccessPointVpcConfigurationElRef {
        S3AccessPointVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3AccessPointVpcConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3AccessPointDynamic {
    public_access_block_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3AccessPointPublicAccessBlockConfigurationEl>>,
    vpc_configuration: std::option::Option<terrars::DynamicBlock<S3AccessPointVpcConfigurationEl>>,
}
