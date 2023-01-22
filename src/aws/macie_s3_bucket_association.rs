use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MacieS3BucketAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_type: Option<Vec<MacieS3BucketAssociationClassificationTypeEl>>,
    dynamic: MacieS3BucketAssociationDynamic,
}

struct MacieS3BucketAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MacieS3BucketAssociationData>,
}

#[derive(Clone)]
pub struct MacieS3BucketAssociation(Rc<MacieS3BucketAssociation_>);

impl MacieS3BucketAssociation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `member_account_id`.\n"]
    pub fn set_member_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().member_account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `classification_type`.\n"]
    pub fn set_classification_type(
        self,
        v: impl Into<BlockAssignable<MacieS3BucketAssociationClassificationTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().classification_type = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.classification_type = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_account_id` after provisioning.\n"]
    pub fn member_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `classification_type` after provisioning.\n"]
    pub fn classification_type(&self) -> ListRef<MacieS3BucketAssociationClassificationTypeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.classification_type", self.extract_ref()))
    }
}

impl Resource for MacieS3BucketAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for MacieS3BucketAssociation {
    type O = ListRef<MacieS3BucketAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MacieS3BucketAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_macie_s3_bucket_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMacieS3BucketAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub bucket_name: PrimField<String>,
}

impl BuildMacieS3BucketAssociation {
    pub fn build(self, stack: &mut Stack) -> MacieS3BucketAssociation {
        let out = MacieS3BucketAssociation(Rc::new(MacieS3BucketAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MacieS3BucketAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_name: self.bucket_name,
                id: core::default::Default::default(),
                member_account_id: core::default::Default::default(),
                prefix: core::default::Default::default(),
                classification_type: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MacieS3BucketAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for MacieS3BucketAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MacieS3BucketAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_account_id` after provisioning.\n"]
    pub fn member_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `classification_type` after provisioning.\n"]
    pub fn classification_type(&self) -> ListRef<MacieS3BucketAssociationClassificationTypeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.classification_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MacieS3BucketAssociationClassificationTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continuous: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time: Option<PrimField<String>>,
}

impl MacieS3BucketAssociationClassificationTypeEl {
    #[doc= "Set the field `continuous`.\n"]
    pub fn set_continuous(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.continuous = Some(v.into());
        self
    }

    #[doc= "Set the field `one_time`.\n"]
    pub fn set_one_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.one_time = Some(v.into());
        self
    }
}

impl ToListMappable for MacieS3BucketAssociationClassificationTypeEl {
    type O = BlockAssignable<MacieS3BucketAssociationClassificationTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacieS3BucketAssociationClassificationTypeEl {}

impl BuildMacieS3BucketAssociationClassificationTypeEl {
    pub fn build(self) -> MacieS3BucketAssociationClassificationTypeEl {
        MacieS3BucketAssociationClassificationTypeEl {
            continuous: core::default::Default::default(),
            one_time: core::default::Default::default(),
        }
    }
}

pub struct MacieS3BucketAssociationClassificationTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MacieS3BucketAssociationClassificationTypeElRef {
    fn new(shared: StackShared, base: String) -> MacieS3BucketAssociationClassificationTypeElRef {
        MacieS3BucketAssociationClassificationTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MacieS3BucketAssociationClassificationTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `continuous` after provisioning.\n"]
    pub fn continuous(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.continuous", self.base))
    }

    #[doc= "Get a reference to the value of field `one_time` after provisioning.\n"]
    pub fn one_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.one_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MacieS3BucketAssociationDynamic {
    classification_type: Option<DynamicBlock<MacieS3BucketAssociationClassificationTypeEl>>,
}
