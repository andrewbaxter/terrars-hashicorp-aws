#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksCapabilityData_ {
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
    capability_name: terrars::PrimField<std::string::String>,
    cluster_name: terrars::PrimField<std::string::String>,
    delete_propagation_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<EksCapability_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EksCapability_Timeouts>,
    dynamic: EksCapabilityDynamic,
}

struct EksCapability_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EksCapabilityData_>,
}

#[derive(Clone)]
pub struct EksCapability(std::rc::Rc<EksCapability_>);

impl EksCapability {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCapability_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EksCapability_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capability_name` after provisioning.\n"]
    pub fn capability_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capability_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_propagation_policy` after provisioning.\n"]
    pub fn delete_propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_propagation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<EksCapability_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksCapability_TimeoutsRef {
        EksCapability_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EksCapability {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EksCapability { }

impl terrars::ToListMappable for EksCapability {
    type O = terrars::ListRef<EksCapabilityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EksCapability_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_eks_capability".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksCapability {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub capability_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub delete_propagation_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEksCapability {
    pub fn build(self, stack: &mut terrars::Stack) -> EksCapability {
        let out = EksCapability(std::rc::Rc::new(EksCapability_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EksCapabilityData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capability_name: self.capability_name,
                cluster_name: self.cluster_name,
                delete_propagation_policy: self.delete_propagation_policy,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                type_: self.type_,
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksCapabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EksCapabilityRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capability_name` after provisioning.\n"]
    pub fn capability_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capability_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_propagation_policy` after provisioning.\n"]
    pub fn delete_propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_propagation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<EksCapability_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksCapability_TimeoutsRef {
        EksCapability_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EksCapabilityDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<EksCapability_Configuration>>,
}

#[derive(Serialize)]
pub struct EksCapability_Configuration_ArgoCd_AwsIdc {
    idc_instance_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idc_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCapability_Configuration_ArgoCd_AwsIdc {
    #[doc = "Set the field `idc_region`.\n"]
    pub fn set_idc_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idc_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCapability_Configuration_ArgoCd_AwsIdc {
    type O = terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_AwsIdc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration_ArgoCd_AwsIdc {
    #[doc = ""]
    pub idc_instance_arn: terrars::PrimField<std::string::String>,
}

impl BuildEksCapability_Configuration_ArgoCd_AwsIdc {
    pub fn build(self) -> EksCapability_Configuration_ArgoCd_AwsIdc {
        EksCapability_Configuration_ArgoCd_AwsIdc {
            idc_instance_arn: self.idc_instance_arn,
            idc_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_Configuration_ArgoCd_AwsIdcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_Configuration_ArgoCd_AwsIdcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCapability_Configuration_ArgoCd_AwsIdcRef {
        EksCapability_Configuration_ArgoCd_AwsIdcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_Configuration_ArgoCd_AwsIdcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idc_instance_arn` after provisioning.\n"]
    pub fn idc_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `idc_managed_application_arn` after provisioning.\n"]
    pub fn idc_managed_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_managed_application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `idc_region` after provisioning.\n"]
    pub fn idc_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idc_region", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCapability_Configuration_ArgoCd_NetworkAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpce_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EksCapability_Configuration_ArgoCd_NetworkAccess {
    #[doc = "Set the field `vpce_ids`.\n"]
    pub fn set_vpce_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vpce_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCapability_Configuration_ArgoCd_NetworkAccess {
    type O = terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_NetworkAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration_ArgoCd_NetworkAccess {}

impl BuildEksCapability_Configuration_ArgoCd_NetworkAccess {
    pub fn build(self) -> EksCapability_Configuration_ArgoCd_NetworkAccess {
        EksCapability_Configuration_ArgoCd_NetworkAccess { vpce_ids: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCapability_Configuration_ArgoCd_NetworkAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_Configuration_ArgoCd_NetworkAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCapability_Configuration_ArgoCd_NetworkAccessRef {
        EksCapability_Configuration_ArgoCd_NetworkAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_Configuration_ArgoCd_NetworkAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpce_ids` after provisioning.\n"]
    pub fn vpce_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpce_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
    id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity { }

impl terrars::ToListMappable for EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
    type O = terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
    pub fn build(self) -> EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
        EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity {
            id: self.id,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_Configuration_ArgoCd_RbacRoleMapping_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_Configuration_ArgoCd_RbacRoleMapping_IdentityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCapability_Configuration_ArgoCd_RbacRoleMapping_IdentityRef {
        EksCapability_Configuration_ArgoCd_RbacRoleMapping_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_Configuration_ArgoCd_RbacRoleMapping_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCapability_Configuration_ArgoCd_RbacRoleMappingDynamic {
    identity: std
    ::option
    ::Option<terrars::DynamicBlock<EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity>>,
}

#[derive(Serialize)]
pub struct EksCapability_Configuration_ArgoCd_RbacRoleMapping {
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<std::vec::Vec<EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity>>,
    dynamic: EksCapability_Configuration_ArgoCd_RbacRoleMappingDynamic,
}

impl EksCapability_Configuration_ArgoCd_RbacRoleMapping {
    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_RbacRoleMapping_Identity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCapability_Configuration_ArgoCd_RbacRoleMapping {
    type O = terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_RbacRoleMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration_ArgoCd_RbacRoleMapping {
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildEksCapability_Configuration_ArgoCd_RbacRoleMapping {
    pub fn build(self) -> EksCapability_Configuration_ArgoCd_RbacRoleMapping {
        EksCapability_Configuration_ArgoCd_RbacRoleMapping {
            role: self.role,
            identity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_Configuration_ArgoCd_RbacRoleMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_Configuration_ArgoCd_RbacRoleMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCapability_Configuration_ArgoCd_RbacRoleMappingRef {
        EksCapability_Configuration_ArgoCd_RbacRoleMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_Configuration_ArgoCd_RbacRoleMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCapability_Configuration_ArgoCdDynamic {
    aws_idc: std::option::Option<terrars::DynamicBlock<EksCapability_Configuration_ArgoCd_AwsIdc>>,
    network_access: std::option::Option<terrars::DynamicBlock<EksCapability_Configuration_ArgoCd_NetworkAccess>>,
    rbac_role_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<EksCapability_Configuration_ArgoCd_RbacRoleMapping>>,
}

#[derive(Serialize)]
pub struct EksCapability_Configuration_ArgoCd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_idc: std::option::Option<std::vec::Vec<EksCapability_Configuration_ArgoCd_AwsIdc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_access: std::option::Option<std::vec::Vec<EksCapability_Configuration_ArgoCd_NetworkAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rbac_role_mapping: std::option::Option<std::vec::Vec<EksCapability_Configuration_ArgoCd_RbacRoleMapping>>,
    dynamic: EksCapability_Configuration_ArgoCdDynamic,
}

impl EksCapability_Configuration_ArgoCd {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_idc`.\n"]
    pub fn set_aws_idc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_AwsIdc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_idc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_idc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_access`.\n"]
    pub fn set_network_access(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_NetworkAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rbac_role_mapping`.\n"]
    pub fn set_rbac_role_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCapability_Configuration_ArgoCd_RbacRoleMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rbac_role_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rbac_role_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCapability_Configuration_ArgoCd {
    type O = terrars::BlockAssignable<EksCapability_Configuration_ArgoCd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration_ArgoCd {}

impl BuildEksCapability_Configuration_ArgoCd {
    pub fn build(self) -> EksCapability_Configuration_ArgoCd {
        EksCapability_Configuration_ArgoCd {
            namespace: core::default::Default::default(),
            aws_idc: core::default::Default::default(),
            network_access: core::default::Default::default(),
            rbac_role_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_Configuration_ArgoCdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_Configuration_ArgoCdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCapability_Configuration_ArgoCdRef {
        EksCapability_Configuration_ArgoCdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_Configuration_ArgoCdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `server_url` after provisioning.\n"]
    pub fn server_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_url", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_idc` after provisioning.\n"]
    pub fn aws_idc(&self) -> terrars::ListRef<EksCapability_Configuration_ArgoCd_AwsIdcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_idc", self.base))
    }

    #[doc = "Get a reference to the value of field `network_access` after provisioning.\n"]
    pub fn network_access(&self) -> terrars::ListRef<EksCapability_Configuration_ArgoCd_NetworkAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_access", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCapability_ConfigurationDynamic {
    argo_cd: std::option::Option<terrars::DynamicBlock<EksCapability_Configuration_ArgoCd>>,
}

#[derive(Serialize)]
pub struct EksCapability_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    argo_cd: std::option::Option<std::vec::Vec<EksCapability_Configuration_ArgoCd>>,
    dynamic: EksCapability_ConfigurationDynamic,
}

impl EksCapability_Configuration {
    #[doc = "Set the field `argo_cd`.\n"]
    pub fn set_argo_cd(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCapability_Configuration_ArgoCd>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.argo_cd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.argo_cd = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCapability_Configuration {
    type O = terrars::BlockAssignable<EksCapability_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Configuration {}

impl BuildEksCapability_Configuration {
    pub fn build(self) -> EksCapability_Configuration {
        EksCapability_Configuration {
            argo_cd: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCapability_ConfigurationRef {
        EksCapability_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `argo_cd` after provisioning.\n"]
    pub fn argo_cd(&self) -> terrars::ListRef<EksCapability_Configuration_ArgoCdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.argo_cd", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCapability_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCapability_Timeouts {
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

impl terrars::ToListMappable for EksCapability_Timeouts {
    type O = terrars::BlockAssignable<EksCapability_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCapability_Timeouts {}

impl BuildEksCapability_Timeouts {
    pub fn build(self) -> EksCapability_Timeouts {
        EksCapability_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCapability_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCapability_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCapability_TimeoutsRef {
        EksCapability_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCapability_TimeoutsRef {
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
