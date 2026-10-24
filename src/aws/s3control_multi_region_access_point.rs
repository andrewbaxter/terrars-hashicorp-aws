use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlMultiRegionAccessPointData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<std::vec::Vec<S3controlMultiRegionAccessPointDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<S3controlMultiRegionAccessPointTimeoutsEl>,
    dynamic: S3controlMultiRegionAccessPointDynamic,
}

struct S3controlMultiRegionAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlMultiRegionAccessPointData>,
}

#[derive(Clone)]
pub struct S3controlMultiRegionAccessPoint(std::rc::Rc<S3controlMultiRegionAccessPoint_>);

impl S3controlMultiRegionAccessPoint {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<S3controlMultiRegionAccessPointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<S3controlMultiRegionAccessPointDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for S3controlMultiRegionAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlMultiRegionAccessPoint { }

impl terrars::ToListMappable for S3controlMultiRegionAccessPoint {
    type O = terrars::ListRef<S3controlMultiRegionAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlMultiRegionAccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_multi_region_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlMultiRegionAccessPoint {
    pub tf_id: std::string::String,
}

impl BuildS3controlMultiRegionAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlMultiRegionAccessPoint {
        let out = S3controlMultiRegionAccessPoint(std::rc::Rc::new(S3controlMultiRegionAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlMultiRegionAccessPointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlMultiRegionAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlMultiRegionAccessPointRef {
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

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<S3controlMultiRegionAccessPointDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrict_public_buckets: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
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

impl terrars::ToListMappable for S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {}

impl BuildS3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
        S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

pub struct S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
        S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
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
pub struct S3controlMultiRegionAccessPointDetailsElRegionEl {
    bucket: terrars::PrimField<std::string::String>,
}

impl S3controlMultiRegionAccessPointDetailsElRegionEl { }

impl terrars::ToListMappable for S3controlMultiRegionAccessPointDetailsElRegionEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsElRegionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointDetailsElRegionEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3controlMultiRegionAccessPointDetailsElRegionEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsElRegionEl {
        S3controlMultiRegionAccessPointDetailsElRegionEl { bucket: self.bucket }
    }
}

pub struct S3controlMultiRegionAccessPointDetailsElRegionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointDetailsElRegionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlMultiRegionAccessPointDetailsElRegionElRef {
        S3controlMultiRegionAccessPointDetailsElRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointDetailsElRegionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointDetailsElDynamic {
    public_access_block: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    region: std::option::Option<terrars::DynamicBlock<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointDetailsEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access_block: std::option::Option<std::vec::Vec<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<std::vec::Vec<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
    dynamic: S3controlMultiRegionAccessPointDetailsElDynamic,
}

impl S3controlMultiRegionAccessPointDetailsEl {
    #[doc = "Set the field `public_access_block`.\n"]
    pub fn set_public_access_block(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_access_block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_access_block = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlMultiRegionAccessPointDetailsEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointDetailsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3controlMultiRegionAccessPointDetailsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsEl {
        S3controlMultiRegionAccessPointDetailsEl {
            name: self.name,
            public_access_block: core::default::Default::default(),
            region: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct S3controlMultiRegionAccessPointDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3controlMultiRegionAccessPointDetailsElRef {
        S3controlMultiRegionAccessPointDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(
        &self,
    ) -> terrars::ListRef<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access_block", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3controlMultiRegionAccessPointTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlMultiRegionAccessPointTimeoutsEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointTimeoutsEl {}

impl BuildS3controlMultiRegionAccessPointTimeoutsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointTimeoutsEl {
        S3controlMultiRegionAccessPointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct S3controlMultiRegionAccessPointTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointDynamic {
    details: std::option::Option<terrars::DynamicBlock<S3controlMultiRegionAccessPointDetailsEl>>,
}
