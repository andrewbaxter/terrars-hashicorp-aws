#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcPeeringConnectionOptionsData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_peering_connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepter: std::option::Option<std::vec::Vec<VpcPeeringConnectionOptions_Accepter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester: std::option::Option<std::vec::Vec<VpcPeeringConnectionOptions_Requester>>,
    dynamic: VpcPeeringConnectionOptionsDynamic,
}

struct VpcPeeringConnectionOptions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcPeeringConnectionOptionsData_>,
}

#[derive(Clone)]
pub struct VpcPeeringConnectionOptions(std::rc::Rc<VpcPeeringConnectionOptions_>);

impl VpcPeeringConnectionOptions {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accepter`.\n"]
    pub fn set_accepter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcPeeringConnectionOptions_Accepter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accepter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accepter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requester`.\n"]
    pub fn set_requester(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcPeeringConnectionOptions_Requester>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requester = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::ListRef<VpcPeeringConnectionOptions_AccepterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::ListRef<VpcPeeringConnectionOptions_RequesterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }
}

impl terrars::Referable for VpcPeeringConnectionOptions {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcPeeringConnectionOptions { }

impl terrars::ToListMappable for VpcPeeringConnectionOptions {
    type O = terrars::ListRef<VpcPeeringConnectionOptionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcPeeringConnectionOptions_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpc_peering_connection_options".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcPeeringConnectionOptions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub vpc_peering_connection_id: terrars::PrimField<std::string::String>,
}

impl BuildVpcPeeringConnectionOptions {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcPeeringConnectionOptions {
        let out = VpcPeeringConnectionOptions(std::rc::Rc::new(VpcPeeringConnectionOptions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcPeeringConnectionOptionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                vpc_peering_connection_id: self.vpc_peering_connection_id,
                accepter: core::default::Default::default(),
                requester: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcPeeringConnectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcPeeringConnectionOptionsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::ListRef<VpcPeeringConnectionOptions_AccepterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::ListRef<VpcPeeringConnectionOptions_RequesterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VpcPeeringConnectionOptionsDynamic {
    accepter: std::option::Option<terrars::DynamicBlock<VpcPeeringConnectionOptions_Accepter>>,
    requester: std::option::Option<terrars::DynamicBlock<VpcPeeringConnectionOptions_Requester>>,
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionOptions_Accepter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_remote_vpc_dns_resolution: std::option::Option<terrars::PrimField<bool>>,
}

impl VpcPeeringConnectionOptions_Accepter {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcPeeringConnectionOptions_Accepter {
    type O = terrars::BlockAssignable<VpcPeeringConnectionOptions_Accepter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionOptions_Accepter {}

impl BuildVpcPeeringConnectionOptions_Accepter {
    pub fn build(self) -> VpcPeeringConnectionOptions_Accepter {
        VpcPeeringConnectionOptions_Accepter { allow_remote_vpc_dns_resolution: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VpcPeeringConnectionOptions_AccepterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionOptions_AccepterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcPeeringConnectionOptions_AccepterRef {
        VpcPeeringConnectionOptions_AccepterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionOptions_AccepterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionOptions_Requester {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_remote_vpc_dns_resolution: std::option::Option<terrars::PrimField<bool>>,
}

impl VpcPeeringConnectionOptions_Requester {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcPeeringConnectionOptions_Requester {
    type O = terrars::BlockAssignable<VpcPeeringConnectionOptions_Requester>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionOptions_Requester {}

impl BuildVpcPeeringConnectionOptions_Requester {
    pub fn build(self) -> VpcPeeringConnectionOptions_Requester {
        VpcPeeringConnectionOptions_Requester { allow_remote_vpc_dns_resolution: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VpcPeeringConnectionOptions_RequesterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionOptions_RequesterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcPeeringConnectionOptions_RequesterRef {
        VpcPeeringConnectionOptions_RequesterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionOptions_RequesterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }
}
