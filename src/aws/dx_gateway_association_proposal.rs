use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxGatewayAssociationProposalData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_prefixes: Option<SetField<PrimField<String>>>,
    associated_gateway_id: PrimField<String>,
    dx_gateway_id: PrimField<String>,
    dx_gateway_owner_account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DxGatewayAssociationProposal_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DxGatewayAssociationProposalData>,
}

#[derive(Clone)]
pub struct DxGatewayAssociationProposal(Rc<DxGatewayAssociationProposal_>);

impl DxGatewayAssociationProposal {
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

    #[doc= "Set the field `allowed_prefixes`.\n"]
    pub fn set_allowed_prefixes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_prefixes = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allowed_prefixes` after provisioning.\n"]
    pub fn allowed_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_prefixes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_id` after provisioning.\n"]
    pub fn associated_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_owner_account_id` after provisioning.\n"]
    pub fn associated_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_type` after provisioning.\n"]
    pub fn associated_gateway_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dx_gateway_owner_account_id` after provisioning.\n"]
    pub fn dx_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Resource for DxGatewayAssociationProposal {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DxGatewayAssociationProposal {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for DxGatewayAssociationProposal {
    type O = ListRef<DxGatewayAssociationProposalRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DxGatewayAssociationProposal_ {
    fn extract_resource_type(&self) -> String {
        "aws_dx_gateway_association_proposal".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxGatewayAssociationProposal {
    pub tf_id: String,
    #[doc= ""]
    pub associated_gateway_id: PrimField<String>,
    #[doc= ""]
    pub dx_gateway_id: PrimField<String>,
    #[doc= ""]
    pub dx_gateway_owner_account_id: PrimField<String>,
}

impl BuildDxGatewayAssociationProposal {
    pub fn build(self, stack: &mut Stack) -> DxGatewayAssociationProposal {
        let out = DxGatewayAssociationProposal(Rc::new(DxGatewayAssociationProposal_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DxGatewayAssociationProposalData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allowed_prefixes: core::default::Default::default(),
                associated_gateway_id: self.associated_gateway_id,
                dx_gateway_id: self.dx_gateway_id,
                dx_gateway_owner_account_id: self.dx_gateway_owner_account_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxGatewayAssociationProposalRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxGatewayAssociationProposalRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DxGatewayAssociationProposalRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_prefixes` after provisioning.\n"]
    pub fn allowed_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_prefixes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_id` after provisioning.\n"]
    pub fn associated_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_owner_account_id` after provisioning.\n"]
    pub fn associated_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `associated_gateway_type` after provisioning.\n"]
    pub fn associated_gateway_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dx_gateway_owner_account_id` after provisioning.\n"]
    pub fn dx_gateway_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
