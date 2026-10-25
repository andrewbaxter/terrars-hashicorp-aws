#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcEncryptionControlData_ {
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
    egress_only_internet_gateway_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_file_system_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_private_gateway_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_lattice_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_exclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpcEncryptionControl_Timeouts>,
}

struct VpcEncryptionControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcEncryptionControlData_>,
}

#[derive(Clone)]
pub struct VpcEncryptionControl(std::rc::Rc<VpcEncryptionControl_>);

impl VpcEncryptionControl {
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

    #[doc = "Set the field `egress_only_internet_gateway_exclusion`.\n"]
    pub fn set_egress_only_internet_gateway_exclusion(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().egress_only_internet_gateway_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_file_system_exclusion`.\n"]
    pub fn set_elastic_file_system_exclusion(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().elastic_file_system_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway_exclusion`.\n"]
    pub fn set_internet_gateway_exclusion(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().internet_gateway_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_exclusion`.\n"]
    pub fn set_lambda_exclusion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lambda_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_exclusion`.\n"]
    pub fn set_nat_gateway_exclusion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nat_gateway_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_private_gateway_exclusion`.\n"]
    pub fn set_virtual_private_gateway_exclusion(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().virtual_private_gateway_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_lattice_exclusion`.\n"]
    pub fn set_vpc_lattice_exclusion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_lattice_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_exclusion`.\n"]
    pub fn set_vpc_peering_exclusion(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_peering_exclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpcEncryptionControl_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_exclusion` after provisioning.\n"]
    pub fn egress_only_internet_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.egress_only_internet_gateway_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elastic_file_system_exclusion` after provisioning.\n"]
    pub fn elastic_file_system_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.elastic_file_system_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internet_gateway_exclusion` after provisioning.\n"]
    pub fn internet_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internet_gateway_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_exclusion` after provisioning.\n"]
    pub fn lambda_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_exclusion` after provisioning.\n"]
    pub fn nat_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_exclusion", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_exclusions` after provisioning.\n"]
    pub fn resource_exclusions(&self) -> VpcEncryptionControl_ResourceExclusionsRef {
        VpcEncryptionControl_ResourceExclusionsRef::new(
            self.shared().clone(),
            format!("{}.resource_exclusions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_private_gateway_exclusion` after provisioning.\n"]
    pub fn virtual_private_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_private_gateway_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_lattice_exclusion` after provisioning.\n"]
    pub fn vpc_lattice_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_lattice_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_exclusion` after provisioning.\n"]
    pub fn vpc_peering_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEncryptionControl_TimeoutsRef {
        VpcEncryptionControl_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VpcEncryptionControl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcEncryptionControl { }

impl terrars::ToListMappable for VpcEncryptionControl {
    type O = terrars::ListRef<VpcEncryptionControlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcEncryptionControl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpc_encryption_control".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcEncryptionControl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildVpcEncryptionControl {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcEncryptionControl {
        let out = VpcEncryptionControl(std::rc::Rc::new(VpcEncryptionControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcEncryptionControlData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                egress_only_internet_gateway_exclusion: core::default::Default::default(),
                elastic_file_system_exclusion: core::default::Default::default(),
                internet_gateway_exclusion: core::default::Default::default(),
                lambda_exclusion: core::default::Default::default(),
                mode: self.mode,
                nat_gateway_exclusion: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_private_gateway_exclusion: core::default::Default::default(),
                vpc_id: self.vpc_id,
                vpc_lattice_exclusion: core::default::Default::default(),
                vpc_peering_exclusion: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcEncryptionControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcEncryptionControlRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_exclusion` after provisioning.\n"]
    pub fn egress_only_internet_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.egress_only_internet_gateway_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elastic_file_system_exclusion` after provisioning.\n"]
    pub fn elastic_file_system_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.elastic_file_system_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internet_gateway_exclusion` after provisioning.\n"]
    pub fn internet_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internet_gateway_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_exclusion` after provisioning.\n"]
    pub fn lambda_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_exclusion` after provisioning.\n"]
    pub fn nat_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_exclusion", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_exclusions` after provisioning.\n"]
    pub fn resource_exclusions(&self) -> VpcEncryptionControl_ResourceExclusionsRef {
        VpcEncryptionControl_ResourceExclusionsRef::new(
            self.shared().clone(),
            format!("{}.resource_exclusions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_private_gateway_exclusion` after provisioning.\n"]
    pub fn virtual_private_gateway_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_private_gateway_exclusion", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_lattice_exclusion` after provisioning.\n"]
    pub fn vpc_lattice_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_lattice_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_exclusion` after provisioning.\n"]
    pub fn vpc_peering_exclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_exclusion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEncryptionControl_TimeoutsRef {
        VpcEncryptionControl_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {}

impl BuildVpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
        VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
        VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_ElasticFileSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_ElasticFileSystem {}

impl BuildVpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
        VpcEncryptionControl_ResourceExclusions_ElasticFileSystem {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
        VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_InternetGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_InternetGateway {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_InternetGateway {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_InternetGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_InternetGateway {}

impl BuildVpcEncryptionControl_ResourceExclusions_InternetGateway {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_InternetGateway {
        VpcEncryptionControl_ResourceExclusions_InternetGateway {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
        VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_Lambda {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_Lambda {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_Lambda {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_Lambda {}

impl BuildVpcEncryptionControl_ResourceExclusions_Lambda {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_Lambda {
        VpcEncryptionControl_ResourceExclusions_Lambda {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_LambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_LambdaRef {
        VpcEncryptionControl_ResourceExclusions_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_NatGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_NatGateway {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_NatGateway {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_NatGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_NatGateway {}

impl BuildVpcEncryptionControl_ResourceExclusions_NatGateway {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_NatGateway {
        VpcEncryptionControl_ResourceExclusions_NatGateway {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
        VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {}

impl BuildVpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
        VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
        VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_VpcLattice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_VpcLattice {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_VpcLattice {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_VpcLattice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_VpcLattice {}

impl BuildVpcEncryptionControl_ResourceExclusions_VpcLattice {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_VpcLattice {
        VpcEncryptionControl_ResourceExclusions_VpcLattice {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
        VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions_VpcPeering {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_ResourceExclusions_VpcPeering {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions_VpcPeering {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions_VpcPeering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions_VpcPeering {}

impl BuildVpcEncryptionControl_ResourceExclusions_VpcPeering {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions_VpcPeering {
        VpcEncryptionControl_ResourceExclusions_VpcPeering {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
        VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_ResourceExclusions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_only_internet_gateway: std::option::Option<VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_file_system: std::option::Option<VpcEncryptionControl_ResourceExclusions_ElasticFileSystem>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std::option::Option<VpcEncryptionControl_ResourceExclusions_InternetGateway>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<VpcEncryptionControl_ResourceExclusions_Lambda>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway: std::option::Option<VpcEncryptionControl_ResourceExclusions_NatGateway>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_private_gateway: std::option::Option<VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_lattice: std::option::Option<VpcEncryptionControl_ResourceExclusions_VpcLattice>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering: std::option::Option<VpcEncryptionControl_ResourceExclusions_VpcPeering>,
}

impl VpcEncryptionControl_ResourceExclusions {
    #[doc = "Set the field `egress_only_internet_gateway`.\n"]
    pub fn set_egress_only_internet_gateway(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGateway>,
    ) -> Self {
        self.egress_only_internet_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_file_system`.\n"]
    pub fn set_elastic_file_system(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_ElasticFileSystem>,
    ) -> Self {
        self.elastic_file_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_InternetGateway>,
    ) -> Self {
        self.internet_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(mut self, v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_Lambda>) -> Self {
        self.lambda = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway`.\n"]
    pub fn set_nat_gateway(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_NatGateway>,
    ) -> Self {
        self.nat_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_private_gateway`.\n"]
    pub fn set_virtual_private_gateway(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_VirtualPrivateGateway>,
    ) -> Self {
        self.virtual_private_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_lattice`.\n"]
    pub fn set_vpc_lattice(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_VpcLattice>,
    ) -> Self {
        self.vpc_lattice = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering`.\n"]
    pub fn set_vpc_peering(
        mut self,
        v: impl std::convert::Into<VpcEncryptionControl_ResourceExclusions_VpcPeering>,
    ) -> Self {
        self.vpc_peering = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_ResourceExclusions {
    type O = terrars::BlockAssignable<VpcEncryptionControl_ResourceExclusions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_ResourceExclusions {}

impl BuildVpcEncryptionControl_ResourceExclusions {
    pub fn build(self) -> VpcEncryptionControl_ResourceExclusions {
        VpcEncryptionControl_ResourceExclusions {
            egress_only_internet_gateway: core::default::Default::default(),
            elastic_file_system: core::default::Default::default(),
            internet_gateway: core::default::Default::default(),
            lambda: core::default::Default::default(),
            nat_gateway: core::default::Default::default(),
            virtual_private_gateway: core::default::Default::default(),
            vpc_lattice: core::default::Default::default(),
            vpc_peering: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_ResourceExclusionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_ResourceExclusionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcEncryptionControl_ResourceExclusionsRef {
        VpcEncryptionControl_ResourceExclusionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_ResourceExclusionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway` after provisioning.\n"]
    pub fn egress_only_internet_gateway(&self) -> VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef {
        VpcEncryptionControl_ResourceExclusions_EgressOnlyInternetGatewayRef::new(
            self.shared().clone(),
            format!("{}.egress_only_internet_gateway", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `elastic_file_system` after provisioning.\n"]
    pub fn elastic_file_system(&self) -> VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef {
        VpcEncryptionControl_ResourceExclusions_ElasticFileSystemRef::new(
            self.shared().clone(),
            format!("{}.elastic_file_system", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> VpcEncryptionControl_ResourceExclusions_InternetGatewayRef {
        VpcEncryptionControl_ResourceExclusions_InternetGatewayRef::new(
            self.shared().clone(),
            format!("{}.internet_gateway", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> VpcEncryptionControl_ResourceExclusions_LambdaRef {
        VpcEncryptionControl_ResourceExclusions_LambdaRef::new(
            self.shared().clone(),
            format!("{}.lambda", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `nat_gateway` after provisioning.\n"]
    pub fn nat_gateway(&self) -> VpcEncryptionControl_ResourceExclusions_NatGatewayRef {
        VpcEncryptionControl_ResourceExclusions_NatGatewayRef::new(
            self.shared().clone(),
            format!("{}.nat_gateway", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_private_gateway` after provisioning.\n"]
    pub fn virtual_private_gateway(&self) -> VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef {
        VpcEncryptionControl_ResourceExclusions_VirtualPrivateGatewayRef::new(
            self.shared().clone(),
            format!("{}.virtual_private_gateway", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_lattice` after provisioning.\n"]
    pub fn vpc_lattice(&self) -> VpcEncryptionControl_ResourceExclusions_VpcLatticeRef {
        VpcEncryptionControl_ResourceExclusions_VpcLatticeRef::new(
            self.shared().clone(),
            format!("{}.vpc_lattice", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering` after provisioning.\n"]
    pub fn vpc_peering(&self) -> VpcEncryptionControl_ResourceExclusions_VpcPeeringRef {
        VpcEncryptionControl_ResourceExclusions_VpcPeeringRef::new(
            self.shared().clone(),
            format!("{}.vpc_peering", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct VpcEncryptionControl_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcEncryptionControl_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcEncryptionControl_Timeouts {
    type O = terrars::BlockAssignable<VpcEncryptionControl_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEncryptionControl_Timeouts {}

impl BuildVpcEncryptionControl_Timeouts {
    pub fn build(self) -> VpcEncryptionControl_Timeouts {
        VpcEncryptionControl_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcEncryptionControl_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcEncryptionControl_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcEncryptionControl_TimeoutsRef {
        VpcEncryptionControl_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEncryptionControl_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
