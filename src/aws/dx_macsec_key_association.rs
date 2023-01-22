use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxMacsecKeyAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cak: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ckn: Option<PrimField<String>>,
    connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
}

struct DxMacsecKeyAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DxMacsecKeyAssociationData>,
}

#[derive(Clone)]
pub struct DxMacsecKeyAssociation(Rc<DxMacsecKeyAssociation_>);

impl DxMacsecKeyAssociation {
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

    #[doc= "Set the field `cak`.\n"]
    pub fn set_cak(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cak = Some(v.into());
        self
    }

    #[doc= "Set the field `ckn`.\n"]
    pub fn set_ckn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ckn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_arn = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cak` after provisioning.\n"]
    pub fn cak(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cak", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ckn` after provisioning.\n"]
    pub fn ckn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ckn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_on` after provisioning.\n"]
    pub fn start_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }
}

impl Resource for DxMacsecKeyAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DxMacsecKeyAssociation {
    type O = ListRef<DxMacsecKeyAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DxMacsecKeyAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_dx_macsec_key_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxMacsecKeyAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub connection_id: PrimField<String>,
}

impl BuildDxMacsecKeyAssociation {
    pub fn build(self, stack: &mut Stack) -> DxMacsecKeyAssociation {
        let out = DxMacsecKeyAssociation(Rc::new(DxMacsecKeyAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DxMacsecKeyAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cak: core::default::Default::default(),
                ckn: core::default::Default::default(),
                connection_id: self.connection_id,
                id: core::default::Default::default(),
                secret_arn: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxMacsecKeyAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxMacsecKeyAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DxMacsecKeyAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cak` after provisioning.\n"]
    pub fn cak(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cak", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ckn` after provisioning.\n"]
    pub fn ckn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ckn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_on` after provisioning.\n"]
    pub fn start_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }
}
