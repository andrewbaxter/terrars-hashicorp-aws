#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailContainerServiceData_ {
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
    is_disabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    power: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    scale: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_registry_access: std::option::Option<std::vec::Vec<LightsailContainerService_PrivateRegistryAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_domain_names: std::option::Option<std::vec::Vec<LightsailContainerService_PublicDomainNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LightsailContainerService_Timeouts>,
    dynamic: LightsailContainerServiceDynamic,
}

struct LightsailContainerService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LightsailContainerServiceData_>,
}

#[derive(Clone)]
pub struct LightsailContainerService(std::rc::Rc<LightsailContainerService_>);

impl LightsailContainerService {
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

    #[doc = "Set the field `is_disabled`.\n"]
    pub fn set_is_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_disabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_registry_access`.\n"]
    pub fn set_private_registry_access(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailContainerService_PrivateRegistryAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_registry_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_registry_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `public_domain_names`.\n"]
    pub fn set_public_domain_names(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailContainerService_PublicDomainNames>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_domain_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_domain_names = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LightsailContainerService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `power` after provisioning.\n"]
    pub fn power(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.power", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `power_id` after provisioning.\n"]
    pub fn power_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.power_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_domain_name` after provisioning.\n"]
    pub fn private_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_registry_access` after provisioning.\n"]
    pub fn private_registry_access(&self) -> terrars::ListRef<LightsailContainerService_PrivateRegistryAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_domain_names` after provisioning.\n"]
    pub fn public_domain_names(&self) -> terrars::ListRef<LightsailContainerService_PublicDomainNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_domain_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerService_TimeoutsRef {
        LightsailContainerService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LightsailContainerService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LightsailContainerService { }

impl terrars::ToListMappable for LightsailContainerService {
    type O = terrars::ListRef<LightsailContainerServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LightsailContainerService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lightsail_container_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailContainerService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub power: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scale: terrars::PrimField<f64>,
}

impl BuildLightsailContainerService {
    pub fn build(self, stack: &mut terrars::Stack) -> LightsailContainerService {
        let out = LightsailContainerService(std::rc::Rc::new(LightsailContainerService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LightsailContainerServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                is_disabled: core::default::Default::default(),
                name: self.name,
                power: self.power,
                region: core::default::Default::default(),
                scale: self.scale,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                private_registry_access: core::default::Default::default(),
                public_domain_names: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailContainerServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LightsailContainerServiceRef {
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

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_disabled` after provisioning.\n"]
    pub fn is_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `power` after provisioning.\n"]
    pub fn power(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.power", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `power_id` after provisioning.\n"]
    pub fn power_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.power_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_domain_name` after provisioning.\n"]
    pub fn private_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_registry_access` after provisioning.\n"]
    pub fn private_registry_access(&self) -> terrars::ListRef<LightsailContainerService_PrivateRegistryAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_registry_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_domain_names` after provisioning.\n"]
    pub fn public_domain_names(&self) -> terrars::ListRef<LightsailContainerService_PublicDomainNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_domain_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerService_TimeoutsRef {
        LightsailContainerService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LightsailContainerServiceDynamic {
    private_registry_access: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailContainerService_PrivateRegistryAccess>>,
    public_domain_names: std::option::Option<terrars::DynamicBlock<LightsailContainerService_PublicDomainNames>>,
}

#[derive(Serialize)]
pub struct LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_active: std::option::Option<terrars::PrimField<bool>>,
}

impl LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_active = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
    type O = terrars::BlockAssignable<LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {}

impl BuildLightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
    pub fn build(self) -> LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
        LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole {
            is_active: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef {
        LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct LightsailContainerService_PrivateRegistryAccessDynamic {
    ecr_image_puller_role: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole>>,
}

#[derive(Serialize)]
pub struct LightsailContainerService_PrivateRegistryAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_puller_role: std
    ::option
    ::Option<std::vec::Vec<LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole>>,
    dynamic: LightsailContainerService_PrivateRegistryAccessDynamic,
}

impl LightsailContainerService_PrivateRegistryAccess {
    #[doc = "Set the field `ecr_image_puller_role`.\n"]
    pub fn set_ecr_image_puller_role(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRole>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_puller_role = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_puller_role = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LightsailContainerService_PrivateRegistryAccess {
    type O = terrars::BlockAssignable<LightsailContainerService_PrivateRegistryAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerService_PrivateRegistryAccess {}

impl BuildLightsailContainerService_PrivateRegistryAccess {
    pub fn build(self) -> LightsailContainerService_PrivateRegistryAccess {
        LightsailContainerService_PrivateRegistryAccess {
            ecr_image_puller_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerService_PrivateRegistryAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerService_PrivateRegistryAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerService_PrivateRegistryAccessRef {
        LightsailContainerService_PrivateRegistryAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerService_PrivateRegistryAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ecr_image_puller_role` after provisioning.\n"]
    pub fn ecr_image_puller_role(
        &self,
    ) -> terrars::ListRef<LightsailContainerService_PrivateRegistryAccess_EcrImagePullerRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecr_image_puller_role", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailContainerService_PublicDomainNames_Certificate {
    certificate_name: terrars::PrimField<std::string::String>,
    domain_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl LightsailContainerService_PublicDomainNames_Certificate { }

impl terrars::ToListMappable for LightsailContainerService_PublicDomainNames_Certificate {
    type O = terrars::BlockAssignable<LightsailContainerService_PublicDomainNames_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerService_PublicDomainNames_Certificate {
    #[doc = ""]
    pub certificate_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildLightsailContainerService_PublicDomainNames_Certificate {
    pub fn build(self) -> LightsailContainerService_PublicDomainNames_Certificate {
        LightsailContainerService_PublicDomainNames_Certificate {
            certificate_name: self.certificate_name,
            domain_names: self.domain_names,
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerService_PublicDomainNames_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerService_PublicDomainNames_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerService_PublicDomainNames_CertificateRef {
        LightsailContainerService_PublicDomainNames_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerService_PublicDomainNames_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_names` after provisioning.\n"]
    pub fn domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct LightsailContainerService_PublicDomainNamesDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<LightsailContainerService_PublicDomainNames_Certificate>>,
}

#[derive(Serialize)]
pub struct LightsailContainerService_PublicDomainNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<LightsailContainerService_PublicDomainNames_Certificate>>,
    dynamic: LightsailContainerService_PublicDomainNamesDynamic,
}

impl LightsailContainerService_PublicDomainNames {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LightsailContainerService_PublicDomainNames_Certificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LightsailContainerService_PublicDomainNames {
    type O = terrars::BlockAssignable<LightsailContainerService_PublicDomainNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerService_PublicDomainNames {}

impl BuildLightsailContainerService_PublicDomainNames {
    pub fn build(self) -> LightsailContainerService_PublicDomainNames {
        LightsailContainerService_PublicDomainNames {
            certificate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerService_PublicDomainNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerService_PublicDomainNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerService_PublicDomainNamesRef {
        LightsailContainerService_PublicDomainNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerService_PublicDomainNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct LightsailContainerService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailContainerService_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailContainerService_Timeouts {
    type O = terrars::BlockAssignable<LightsailContainerService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerService_Timeouts {}

impl BuildLightsailContainerService_Timeouts {
    pub fn build(self) -> LightsailContainerService_Timeouts {
        LightsailContainerService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailContainerService_TimeoutsRef {
        LightsailContainerService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerService_TimeoutsRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
