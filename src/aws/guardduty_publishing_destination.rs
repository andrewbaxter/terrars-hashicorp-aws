use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyPublishingDestinationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    destination_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_type: Option<PrimField<String>>,
    detector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kms_key_arn: PrimField<String>,
}

struct GuarddutyPublishingDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyPublishingDestinationData>,
}

#[derive(Clone)]
pub struct GuarddutyPublishingDestination(Rc<GuarddutyPublishingDestination_>);

impl GuarddutyPublishingDestination {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `destination_type`.\n"]
    pub fn set_destination_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destination_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }
}

impl Referable for GuarddutyPublishingDestination {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GuarddutyPublishingDestination { }

impl ToListMappable for GuarddutyPublishingDestination {
    type O = ListRef<GuarddutyPublishingDestinationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GuarddutyPublishingDestination_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_publishing_destination".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGuarddutyPublishingDestination {
    pub tf_id: String,
    #[doc= ""]
    pub destination_arn: PrimField<String>,
    #[doc= ""]
    pub detector_id: PrimField<String>,
    #[doc= ""]
    pub kms_key_arn: PrimField<String>,
}

impl BuildGuarddutyPublishingDestination {
    pub fn build(self, stack: &mut Stack) -> GuarddutyPublishingDestination {
        let out = GuarddutyPublishingDestination(Rc::new(GuarddutyPublishingDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GuarddutyPublishingDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                destination_arn: self.destination_arn,
                destination_type: core::default::Default::default(),
                detector_id: self.detector_id,
                id: core::default::Default::default(),
                kms_key_arn: self.kms_key_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GuarddutyPublishingDestinationRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyPublishingDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GuarddutyPublishingDestinationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }
}
