use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksUserProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_self_management: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_public_key: Option<PrimField<String>>,
    ssh_username: PrimField<String>,
    user_arn: PrimField<String>,
}

struct OpsworksUserProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksUserProfileData>,
}

#[derive(Clone)]
pub struct OpsworksUserProfile(Rc<OpsworksUserProfile_>);

impl OpsworksUserProfile {
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

    #[doc= "Set the field `allow_self_management`.\n"]
    pub fn set_allow_self_management(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_self_management = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_public_key`.\n"]
    pub fn set_ssh_public_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssh_public_key = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allow_self_management` after provisioning.\n"]
    pub fn allow_self_management(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_self_management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_public_key` after provisioning.\n"]
    pub fn ssh_public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_username` after provisioning.\n"]
    pub fn ssh_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_arn` after provisioning.\n"]
    pub fn user_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_arn", self.extract_ref()))
    }
}

impl Referable for OpsworksUserProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OpsworksUserProfile { }

impl ToListMappable for OpsworksUserProfile {
    type O = ListRef<OpsworksUserProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpsworksUserProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_user_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksUserProfile {
    pub tf_id: String,
    #[doc= ""]
    pub ssh_username: PrimField<String>,
    #[doc= ""]
    pub user_arn: PrimField<String>,
}

impl BuildOpsworksUserProfile {
    pub fn build(self, stack: &mut Stack) -> OpsworksUserProfile {
        let out = OpsworksUserProfile(Rc::new(OpsworksUserProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksUserProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_self_management: core::default::Default::default(),
                id: core::default::Default::default(),
                ssh_public_key: core::default::Default::default(),
                ssh_username: self.ssh_username,
                user_arn: self.user_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksUserProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksUserProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksUserProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_self_management` after provisioning.\n"]
    pub fn allow_self_management(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_self_management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_public_key` after provisioning.\n"]
    pub fn ssh_public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_username` after provisioning.\n"]
    pub fn ssh_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_arn` after provisioning.\n"]
    pub fn user_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_arn", self.extract_ref()))
    }
}
