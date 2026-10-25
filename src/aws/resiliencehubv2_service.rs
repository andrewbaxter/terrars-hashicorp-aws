#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Resiliencehubv2ServiceData_ {
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
    dependency_discovery: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    regions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_system: std::option::Option<std::vec::Vec<Resiliencehubv2Service_AssociatedSystem>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission_model: std::option::Option<std::vec::Vec<Resiliencehubv2Service_PermissionModel>>,
    dynamic: Resiliencehubv2ServiceDynamic,
}

struct Resiliencehubv2Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Resiliencehubv2ServiceData_>,
}

#[derive(Clone)]
pub struct Resiliencehubv2Service(std::rc::Rc<Resiliencehubv2Service_>);

impl Resiliencehubv2Service {
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

    #[doc = "Set the field `dependency_discovery`.\n"]
    pub fn set_dependency_discovery(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dependency_discovery = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_arn`.\n"]
    pub fn set_policy_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `associated_system`.\n"]
    pub fn set_associated_system(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resiliencehubv2Service_AssociatedSystem>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().associated_system = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.associated_system = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `permission_model`.\n"]
    pub fn set_permission_model(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resiliencehubv2Service_PermissionModel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permission_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permission_model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dependency_discovery` after provisioning.\n"]
    pub fn dependency_discovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dependency_discovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::ListRef<Resiliencehubv2Service_PermissionModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }
}

impl terrars::Referable for Resiliencehubv2Service {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Resiliencehubv2Service { }

impl terrars::ToListMappable for Resiliencehubv2Service {
    type O = terrars::ListRef<Resiliencehubv2ServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Resiliencehubv2Service_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_resiliencehubv2_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResiliencehubv2Service {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildResiliencehubv2Service {
    pub fn build(self, stack: &mut terrars::Stack) -> Resiliencehubv2Service {
        let out = Resiliencehubv2Service(std::rc::Rc::new(Resiliencehubv2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Resiliencehubv2ServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dependency_discovery: core::default::Default::default(),
                description: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                policy_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                regions: self.regions,
                tags: core::default::Default::default(),
                associated_system: core::default::Default::default(),
                permission_model: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Resiliencehubv2ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Resiliencehubv2ServiceRef {
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

    #[doc = "Get a reference to the value of field `dependency_discovery` after provisioning.\n"]
    pub fn dependency_discovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dependency_discovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::ListRef<Resiliencehubv2Service_PermissionModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Resiliencehubv2ServiceDynamic {
    associated_system: std::option::Option<terrars::DynamicBlock<Resiliencehubv2Service_AssociatedSystem>>,
    permission_model: std::option::Option<terrars::DynamicBlock<Resiliencehubv2Service_PermissionModel>>,
}

#[derive(Serialize)]
pub struct Resiliencehubv2Service_AssociatedSystem {
    system_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_journey_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl Resiliencehubv2Service_AssociatedSystem {
    #[doc = "Set the field `user_journey_ids`.\n"]
    pub fn set_user_journey_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_journey_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2Service_AssociatedSystem {
    type O = terrars::BlockAssignable<Resiliencehubv2Service_AssociatedSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2Service_AssociatedSystem {
    #[doc = ""]
    pub system_arn: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubv2Service_AssociatedSystem {
    pub fn build(self) -> Resiliencehubv2Service_AssociatedSystem {
        Resiliencehubv2Service_AssociatedSystem {
            system_arn: self.system_arn,
            user_journey_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2Service_AssociatedSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2Service_AssociatedSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Resiliencehubv2Service_AssociatedSystemRef {
        Resiliencehubv2Service_AssociatedSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2Service_AssociatedSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `system_arn` after provisioning.\n"]
    pub fn system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_journey_ids` after provisioning.\n"]
    pub fn user_journey_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_journey_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct Resiliencehubv2Service_PermissionModel_CrossAccountRole {
    cross_account_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Resiliencehubv2Service_PermissionModel_CrossAccountRole {
    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2Service_PermissionModel_CrossAccountRole {
    type O = terrars::BlockAssignable<Resiliencehubv2Service_PermissionModel_CrossAccountRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2Service_PermissionModel_CrossAccountRole {
    #[doc = ""]
    pub cross_account_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubv2Service_PermissionModel_CrossAccountRole {
    pub fn build(self) -> Resiliencehubv2Service_PermissionModel_CrossAccountRole {
        Resiliencehubv2Service_PermissionModel_CrossAccountRole {
            cross_account_role_arn: self.cross_account_role_arn,
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
        Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role_arn` after provisioning.\n"]
    pub fn cross_account_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Resiliencehubv2Service_PermissionModelDynamic {
    cross_account_role: std
    ::option
    ::Option<terrars::DynamicBlock<Resiliencehubv2Service_PermissionModel_CrossAccountRole>>,
}

#[derive(Serialize)]
pub struct Resiliencehubv2Service_PermissionModel {
    invoker_role_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std::option::Option<std::vec::Vec<Resiliencehubv2Service_PermissionModel_CrossAccountRole>>,
    dynamic: Resiliencehubv2Service_PermissionModelDynamic,
}

impl Resiliencehubv2Service_PermissionModel {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Resiliencehubv2Service_PermissionModel_CrossAccountRole>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_account_role = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_account_role = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Resiliencehubv2Service_PermissionModel {
    type O = terrars::BlockAssignable<Resiliencehubv2Service_PermissionModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubv2Service_PermissionModel {
    #[doc = ""]
    pub invoker_role_name: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubv2Service_PermissionModel {
    pub fn build(self) -> Resiliencehubv2Service_PermissionModel {
        Resiliencehubv2Service_PermissionModel {
            invoker_role_name: self.invoker_role_name,
            cross_account_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Resiliencehubv2Service_PermissionModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resiliencehubv2Service_PermissionModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Resiliencehubv2Service_PermissionModelRef {
        Resiliencehubv2Service_PermissionModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resiliencehubv2Service_PermissionModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invoker_role_name` after provisioning.\n"]
    pub fn invoker_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoker_role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::ListRef<Resiliencehubv2Service_PermissionModel_CrossAccountRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }
}
