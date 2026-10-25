#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchAuthorizeVpcEndpointAccessData_ {
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
    account: terrars::PrimField<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct OpensearchAuthorizeVpcEndpointAccess_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchAuthorizeVpcEndpointAccessData_>,
}

#[derive(Clone)]
pub struct OpensearchAuthorizeVpcEndpointAccess(std::rc::Rc<OpensearchAuthorizeVpcEndpointAccess_>);

impl OpensearchAuthorizeVpcEndpointAccess {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorized_principal` after provisioning.\n"]
    pub fn authorized_principal(&self) -> terrars::ListRef<OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for OpensearchAuthorizeVpcEndpointAccess {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpensearchAuthorizeVpcEndpointAccess { }

impl terrars::ToListMappable for OpensearchAuthorizeVpcEndpointAccess {
    type O = terrars::ListRef<OpensearchAuthorizeVpcEndpointAccessRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpensearchAuthorizeVpcEndpointAccess_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opensearch_authorize_vpc_endpoint_access".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchAuthorizeVpcEndpointAccess {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchAuthorizeVpcEndpointAccess {
    pub fn build(self, stack: &mut terrars::Stack) -> OpensearchAuthorizeVpcEndpointAccess {
        let out = OpensearchAuthorizeVpcEndpointAccess(std::rc::Rc::new(OpensearchAuthorizeVpcEndpointAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpensearchAuthorizeVpcEndpointAccessData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account: self.account,
                domain_name: self.domain_name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchAuthorizeVpcEndpointAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchAuthorizeVpcEndpointAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchAuthorizeVpcEndpointAccessRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorized_principal` after provisioning.\n"]
    pub fn authorized_principal(&self) -> terrars::ListRef<OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal_type`.\n"]
    pub fn set_principal_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
    type O = terrars::BlockAssignable<OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {}

impl BuildOpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
    pub fn build(self) -> OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
        OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipal {
            principal: core::default::Default::default(),
            principal_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef {
        OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchAuthorizeVpcEndpointAccess_AuthorizedPrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.base))
    }
}
