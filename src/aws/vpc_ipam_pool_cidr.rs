use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcIpamPoolCidrData {
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
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    ipam_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_authorization_context: std::option::Option<std::vec::Vec<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpcIpamPoolCidrTimeoutsEl>,
    dynamic: VpcIpamPoolCidrDynamic,
}

struct VpcIpamPoolCidr_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcIpamPoolCidrData>,
}

#[derive(Clone)]
pub struct VpcIpamPoolCidr(std::rc::Rc<VpcIpamPoolCidr_>);

impl VpcIpamPoolCidr {
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

    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_authorization_context`.\n"]
    pub fn set_cidr_authorization_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_authorization_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_authorization_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpcIpamPoolCidrTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_authorization_context` after provisioning.\n"]
    pub fn cidr_authorization_context(&self) -> terrars::ListRef<VpcIpamPoolCidrCidrAuthorizationContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_authorization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VpcIpamPoolCidr {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcIpamPoolCidr { }

impl terrars::ToListMappable for VpcIpamPoolCidr {
    type O = terrars::ListRef<VpcIpamPoolCidrRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcIpamPoolCidr_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpc_ipam_pool_cidr".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcIpamPoolCidr {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub ipam_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildVpcIpamPoolCidr {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcIpamPoolCidr {
        let out = VpcIpamPoolCidr(std::rc::Rc::new(VpcIpamPoolCidr_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcIpamPoolCidrData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cidr: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                cidr_authorization_context: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcIpamPoolCidrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamPoolCidrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcIpamPoolCidrRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_authorization_context` after provisioning.\n"]
    pub fn cidr_authorization_context(&self) -> terrars::ListRef<VpcIpamPoolCidrCidrAuthorizationContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_authorization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcIpamPoolCidrCidrAuthorizationContextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signature: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcIpamPoolCidrCidrAuthorizationContextEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signature`.\n"]
    pub fn set_signature(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signature = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcIpamPoolCidrCidrAuthorizationContextEl {
    type O = terrars::BlockAssignable<VpcIpamPoolCidrCidrAuthorizationContextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamPoolCidrCidrAuthorizationContextEl {}

impl BuildVpcIpamPoolCidrCidrAuthorizationContextEl {
    pub fn build(self) -> VpcIpamPoolCidrCidrAuthorizationContextEl {
        VpcIpamPoolCidrCidrAuthorizationContextEl {
            message: core::default::Default::default(),
            signature: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamPoolCidrCidrAuthorizationContextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamPoolCidrCidrAuthorizationContextElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcIpamPoolCidrCidrAuthorizationContextElRef {
        VpcIpamPoolCidrCidrAuthorizationContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamPoolCidrCidrAuthorizationContextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `signature` after provisioning.\n"]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcIpamPoolCidrTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcIpamPoolCidrTimeoutsEl {
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

impl terrars::ToListMappable for VpcIpamPoolCidrTimeoutsEl {
    type O = terrars::BlockAssignable<VpcIpamPoolCidrTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamPoolCidrTimeoutsEl {}

impl BuildVpcIpamPoolCidrTimeoutsEl {
    pub fn build(self) -> VpcIpamPoolCidrTimeoutsEl {
        VpcIpamPoolCidrTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamPoolCidrTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamPoolCidrTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamPoolCidrTimeoutsElRef {
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
struct VpcIpamPoolCidrDynamic {
    cidr_authorization_context: std
    ::option
    ::Option<terrars::DynamicBlock<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
}
