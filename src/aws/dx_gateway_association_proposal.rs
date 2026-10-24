use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxGatewayAssociationProposalData {
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
    allowed_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    associated_gateway_id: terrars::PrimField<std::string::String>,
    dx_gateway_id: terrars::PrimField<std::string::String>,
    dx_gateway_owner_account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DxGatewayAssociationProposal_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DxGatewayAssociationProposalData>,
}

#[derive(Clone)]
pub struct DxGatewayAssociationProposal(std::rc::Rc<DxGatewayAssociationProposal_>);

impl DxGatewayAssociationProposal {
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

    #[doc = "Set the field `allowed_prefixes`.\n"]
    pub fn set_allowed_prefixes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allowed_prefixes` after provisioning.\n"]
    pub fn allowed_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_gateway_id` after provisioning.\n"]
    pub fn associated_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_gateway_owner_account_id` after provisioning.\n"]
    pub fn associated_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.associated_gateway_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `associated_gateway_type` after provisioning.\n"]
    pub fn associated_gateway_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dx_gateway_owner_account_id` after provisioning.\n"]
    pub fn dx_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DxGatewayAssociationProposal {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DxGatewayAssociationProposal { }

impl terrars::ToListMappable for DxGatewayAssociationProposal {
    type O = terrars::ListRef<DxGatewayAssociationProposalRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DxGatewayAssociationProposal_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dx_gateway_association_proposal".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxGatewayAssociationProposal {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub associated_gateway_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub dx_gateway_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub dx_gateway_owner_account_id: terrars::PrimField<std::string::String>,
}

impl BuildDxGatewayAssociationProposal {
    pub fn build(self, stack: &mut terrars::Stack) -> DxGatewayAssociationProposal {
        let out = DxGatewayAssociationProposal(std::rc::Rc::new(DxGatewayAssociationProposal_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DxGatewayAssociationProposalData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DxGatewayAssociationProposalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DxGatewayAssociationProposalRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_prefixes` after provisioning.\n"]
    pub fn allowed_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_gateway_id` after provisioning.\n"]
    pub fn associated_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_gateway_owner_account_id` after provisioning.\n"]
    pub fn associated_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.associated_gateway_owner_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `associated_gateway_type` after provisioning.\n"]
    pub fn associated_gateway_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_gateway_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dx_gateway_owner_account_id` after provisioning.\n"]
    pub fn dx_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
