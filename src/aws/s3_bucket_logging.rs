use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketLoggingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    target_bucket: PrimField<String>,
    target_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_grant: Option<Vec<S3BucketLoggingTargetGrantEl>>,
    dynamic: S3BucketLoggingDynamic,
}

struct S3BucketLogging_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketLoggingData>,
}

#[derive(Clone)]
pub struct S3BucketLogging(Rc<S3BucketLogging_>);

impl S3BucketLogging {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `target_grant`.\n"]
    pub fn set_target_grant(self, v: impl Into<BlockAssignable<S3BucketLoggingTargetGrantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_grant = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_grant = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_bucket` after provisioning.\n"]
    pub fn target_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_prefix` after provisioning.\n"]
    pub fn target_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_prefix", self.extract_ref()))
    }
}

impl Resource for S3BucketLogging {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for S3BucketLogging {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for S3BucketLogging {
    type O = ListRef<S3BucketLoggingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketLogging_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_logging".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketLogging {
    pub tf_id: String,
    #[doc= ""]
    pub bucket: PrimField<String>,
    #[doc= ""]
    pub target_bucket: PrimField<String>,
    #[doc= ""]
    pub target_prefix: PrimField<String>,
}

impl BuildS3BucketLogging {
    pub fn build(self, stack: &mut Stack) -> S3BucketLogging {
        let out = S3BucketLogging(Rc::new(S3BucketLogging_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketLoggingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                target_bucket: self.target_bucket,
                target_prefix: self.target_prefix,
                target_grant: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketLoggingRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLoggingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketLoggingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_bucket` after provisioning.\n"]
    pub fn target_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_prefix` after provisioning.\n"]
    pub fn target_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_prefix", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketLoggingTargetGrantElGranteeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}

impl S3BucketLoggingTargetGrantElGranteeEl {
    #[doc= "Set the field `email_address`.\n"]
    pub fn set_email_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_address = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLoggingTargetGrantElGranteeEl {
    type O = BlockAssignable<S3BucketLoggingTargetGrantElGranteeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLoggingTargetGrantElGranteeEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildS3BucketLoggingTargetGrantElGranteeEl {
    pub fn build(self) -> S3BucketLoggingTargetGrantElGranteeEl {
        S3BucketLoggingTargetGrantElGranteeEl {
            email_address: core::default::Default::default(),
            id: core::default::Default::default(),
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLoggingTargetGrantElGranteeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLoggingTargetGrantElGranteeElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLoggingTargetGrantElGranteeElRef {
        S3BucketLoggingTargetGrantElGranteeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLoggingTargetGrantElGranteeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc= "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLoggingTargetGrantElDynamic {
    grantee: Option<DynamicBlock<S3BucketLoggingTargetGrantElGranteeEl>>,
}

#[derive(Serialize)]
pub struct S3BucketLoggingTargetGrantEl {
    permission: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee: Option<Vec<S3BucketLoggingTargetGrantElGranteeEl>>,
    dynamic: S3BucketLoggingTargetGrantElDynamic,
}

impl S3BucketLoggingTargetGrantEl {
    #[doc= "Set the field `grantee`.\n"]
    pub fn set_grantee(mut self, v: impl Into<BlockAssignable<S3BucketLoggingTargetGrantElGranteeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grantee = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grantee = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketLoggingTargetGrantEl {
    type O = BlockAssignable<S3BucketLoggingTargetGrantEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLoggingTargetGrantEl {
    #[doc= ""]
    pub permission: PrimField<String>,
}

impl BuildS3BucketLoggingTargetGrantEl {
    pub fn build(self) -> S3BucketLoggingTargetGrantEl {
        S3BucketLoggingTargetGrantEl {
            permission: self.permission,
            grantee: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketLoggingTargetGrantElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLoggingTargetGrantElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLoggingTargetGrantElRef {
        S3BucketLoggingTargetGrantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLoggingTargetGrantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> ListRef<S3BucketLoggingTargetGrantElGranteeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLoggingDynamic {
    target_grant: Option<DynamicBlock<S3BucketLoggingTargetGrantEl>>,
}
