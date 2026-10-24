use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MacieS3BucketAssociationData {
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
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification_type: std::option::Option<std::vec::Vec<MacieS3BucketAssociationClassificationTypeEl>>,
    dynamic: MacieS3BucketAssociationDynamic,
}

struct MacieS3BucketAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MacieS3BucketAssociationData>,
}

#[derive(Clone)]
pub struct MacieS3BucketAssociation(std::rc::Rc<MacieS3BucketAssociation_>);

impl MacieS3BucketAssociation {
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

    #[doc = "Set the field `member_account_id`.\n"]
    pub fn set_member_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().member_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `classification_type`.\n"]
    pub fn set_classification_type(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MacieS3BucketAssociationClassificationTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().classification_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.classification_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_account_id` after provisioning.\n"]
    pub fn member_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.member_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `classification_type` after provisioning.\n"]
    pub fn classification_type(&self) -> terrars::ListRef<MacieS3BucketAssociationClassificationTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classification_type", self.extract_ref()))
    }
}

impl terrars::Referable for MacieS3BucketAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MacieS3BucketAssociation { }

impl terrars::ToListMappable for MacieS3BucketAssociation {
    type O = terrars::ListRef<MacieS3BucketAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MacieS3BucketAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_macie_s3_bucket_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMacieS3BucketAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildMacieS3BucketAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> MacieS3BucketAssociation {
        let out = MacieS3BucketAssociation(std::rc::Rc::new(MacieS3BucketAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MacieS3BucketAssociationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket_name: self.bucket_name,
                id: core::default::Default::default(),
                member_account_id: core::default::Default::default(),
                prefix: core::default::Default::default(),
                classification_type: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MacieS3BucketAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MacieS3BucketAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MacieS3BucketAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_account_id` after provisioning.\n"]
    pub fn member_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.member_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `classification_type` after provisioning.\n"]
    pub fn classification_type(&self) -> terrars::ListRef<MacieS3BucketAssociationClassificationTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classification_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MacieS3BucketAssociationClassificationTypeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    one_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MacieS3BucketAssociationClassificationTypeEl {
    #[doc = "Set the field `continuous`.\n"]
    pub fn set_continuous(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.continuous = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `one_time`.\n"]
    pub fn set_one_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.one_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MacieS3BucketAssociationClassificationTypeEl {
    type O = terrars::BlockAssignable<MacieS3BucketAssociationClassificationTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MacieS3BucketAssociationClassificationTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MacieS3BucketAssociationClassificationTypeElRef {
        MacieS3BucketAssociationClassificationTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MacieS3BucketAssociationClassificationTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `continuous` after provisioning.\n"]
    pub fn continuous(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continuous", self.base))
    }

    #[doc = "Get a reference to the value of field `one_time` after provisioning.\n"]
    pub fn one_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.one_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct MacieS3BucketAssociationDynamic {
    classification_type: std::option::Option<terrars::DynamicBlock<MacieS3BucketAssociationClassificationTypeEl>>,
}
