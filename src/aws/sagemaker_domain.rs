#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDomainData_ {
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
    app_network_access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_security_group_management: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_mode: terrars::PrimField<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_propagation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_space_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultSpaceSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_user_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DomainSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<SagemakerDomain_RetentionPolicy>>,
    dynamic: SagemakerDomainDynamic,
}

struct SagemakerDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerDomainData_>,
}

#[derive(Clone)]
pub struct SagemakerDomain(std::rc::Rc<SagemakerDomain_>);

impl SagemakerDomain {
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

    #[doc = "Set the field `app_network_access_type`.\n"]
    pub fn set_app_network_access_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().app_network_access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_security_group_management`.\n"]
    pub fn set_app_security_group_management(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().app_security_group_management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_propagation`.\n"]
    pub fn set_tag_propagation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tag_propagation = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `default_space_settings`.\n"]
    pub fn set_default_space_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DefaultSpaceSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_space_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_space_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_user_settings`.\n"]
    pub fn set_default_user_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_user_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_user_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain_settings`.\n"]
    pub fn set_domain_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DomainSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_RetentionPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_security_group_management", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id_for_domain_boundary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_application_arn` after provisioning.\n"]
    pub fn single_sign_on_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_application_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation` after provisioning.\n"]
    pub fn tag_propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultSpaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultUserSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> terrars::ListRef<SagemakerDomain_DomainSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<SagemakerDomain_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerDomain { }

impl terrars::ToListMappable for SagemakerDomain {
    type O = terrars::ListRef<SagemakerDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub auth_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerDomain {
        let out = SagemakerDomain(std::rc::Rc::new(SagemakerDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_network_access_type: core::default::Default::default(),
                app_security_group_management: core::default::Default::default(),
                auth_mode: self.auth_mode,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tag_propagation: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                default_space_settings: core::default::Default::default(),
                default_user_settings: core::default::Default::default(),
                domain_settings: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerDomainRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_security_group_management", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id_for_domain_boundary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_application_arn` after provisioning.\n"]
    pub fn single_sign_on_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_application_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation` after provisioning.\n"]
    pub fn tag_propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultSpaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultUserSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> terrars::ListRef<SagemakerDomain_DomainSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<SagemakerDomain_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDynamic {
    default_space_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomain_DefaultSpaceSettings>>,
    default_user_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings>>,
    domain_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomain_DomainSettings>>,
    retention_policy: std::option::Option<terrars::DynamicBlock<SagemakerDomain_RetentionPolicy>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    file_system_id: terrars::PrimField<std::string::String>,
    file_system_path: terrars::PrimField<std::string::String>,
}

impl SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig { }

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    type O =
        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
        SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
            file_system_id: self.file_system_id,
            file_system_path: self.file_system_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
        SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigDynamic {
    efs_file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig>>,
    dynamic: SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigDynamic,
}

impl SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
    #[doc = "Set the field `efs_file_system_config`.\n"]
    pub fn set_efs_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {}

impl BuildSagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
        SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig {
            efs_file_system_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef {
        SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system_config` after provisioning.\n"]
    pub fn efs_file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_system_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
    gid: terrars::PrimField<f64>,
    uid: terrars::PrimField<f64>,
}

impl SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig { }

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
    #[doc = ""]
    pub gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub uid: terrars::PrimField<f64>,
}

impl BuildSagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
        SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig {
            gid: self.gid,
            uid: self.uid,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef {
        SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_management: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    pub fn build(
        self,
    ) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic {
    idle_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_settings: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>,
    >,
    dynamic: SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic,
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    type O =
        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {}

impl BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement {
            idle_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> terrars
    ::ListRef<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository { }

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository {
            repository_url: self.repository_url,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepositoryRef {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {}

impl BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assumable_role_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
    #[doc = "Set the field `assumable_role_arns`.\n"]
    pub fn set_assumable_role_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.assumable_role_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arns`.\n"]
    pub fn set_execution_role_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.execution_role_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings {
            assumable_role_arns: core::default::Default::default(),
            execution_role_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assumable_role_arns` after provisioning.\n"]
    pub fn assumable_role_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.assumable_role_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arns` after provisioning.\n"]
    pub fn execution_role_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.execution_role_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>>,
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    emr_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    built_in_lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings>>,
    dynamic: SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.built_in_lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_settings`.\n"]
    pub fn set_emr_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.emr_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            emr_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_settings` after provisioning.\n"]
    pub fn emr_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_EmrSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emr_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings_CodeRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
        SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
        SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    default_ebs_volume_size_in_gb: terrars::PrimField<f64>,
    maximum_ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings { }

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    type O =
        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    #[doc = ""]
    pub default_ebs_volume_size_in_gb: terrars::PrimField<f64>,
    #[doc = ""]
    pub maximum_ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl BuildSagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
        SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
            default_ebs_volume_size_in_gb: self.default_ebs_volume_size_in_gb,
            maximum_ebs_volume_size_in_gb: self.maximum_ebs_volume_size_in_gb,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
        SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn default_ebs_volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ebs_volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn maximum_ebs_volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_ebs_volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsDynamic {
    default_ebs_storage_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ebs_storage_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>>,
    dynamic: SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
    #[doc = "Set the field `default_ebs_storage_settings`.\n"]
    pub fn set_default_ebs_storage_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_ebs_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ebs_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_SpaceStorageSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
        SagemakerDomain_DefaultUserSettings_SpaceStorageSettings {
            default_ebs_storage_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef {
        SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_storage_settings` after provisioning.\n"]
    pub fn default_ebs_storage_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_ebs_storage_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultSpaceSettingsDynamic {
    custom_file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    custom_posix_user_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    jupyter_lab_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    space_storage_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultSpaceSettings {
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_posix_user_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_lab_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_storage_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
    dynamic: SagemakerDomain_DefaultSpaceSettingsDynamic,
}

impl SagemakerDomain_DefaultSpaceSettings {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.space_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultSpaceSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultSpaceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultSpaceSettings {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultSpaceSettings {
    pub fn build(self) -> SagemakerDomain_DefaultSpaceSettings {
        SagemakerDomain_DefaultSpaceSettings {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultSpaceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultSpaceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomain_DefaultSpaceSettingsRef {
        SagemakerDomain_DefaultSpaceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultSpaceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_posix_user_config", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings {
            execution_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_bedrock_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
    #[doc = "Set the field `amazon_bedrock_role_arn`.\n"]
    pub fn set_amazon_bedrock_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.amazon_bedrock_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings {
            amazon_bedrock_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_bedrock_role_arn` after provisioning.\n"]
    pub fn amazon_bedrock_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_bedrock_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_name: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[doc = "Set the field `data_source_name`.\n"]
    pub fn set_data_source_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
            data_source_name: core::default::Default::default(),
            secret_arn: self.secret_arn,
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_model_register_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
    #[doc = "Set the field `cross_account_model_register_role_arn`.\n"]
    pub fn set_cross_account_model_register_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_model_register_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings {
            cross_account_model_register_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_model_register_role_arn` after provisioning.\n"]
    pub fn cross_account_model_register_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_model_register_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_forecast_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    #[doc = "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.amazon_forecast_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_forecast_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_artifact_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
    #[doc = "Set the field `s3_artifact_path`.\n"]
    pub fn set_s3_artifact_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_artifact_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings {
            s3_artifact_path: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_artifact_path` after provisioning.\n"]
    pub fn s3_artifact_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_artifact_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_CanvasAppSettingsDynamic {
    direct_deploy_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings>>,
    emr_serverless_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings>>,
    generative_ai_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings>>,
    identity_provider_oauth_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings>,
    >,
    kendra_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings>>,
    model_register_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings>>,
    time_series_forecasting_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>,
    >,
    workspace_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_deploy_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_serverless_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_ai_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_oauth_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kendra_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_register_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_series_forecasting_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings>>,
    dynamic: SagemakerDomain_DefaultUserSettings_CanvasAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettings {
    #[doc = "Set the field `direct_deploy_settings`.\n"]
    pub fn set_direct_deploy_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.direct_deploy_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_deploy_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_serverless_settings`.\n"]
    pub fn set_emr_serverless_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.emr_serverless_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_serverless_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generative_ai_settings`.\n"]
    pub fn set_generative_ai_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generative_ai_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_ai_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_provider_oauth_settings`.\n"]
    pub fn set_identity_provider_oauth_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_provider_oauth_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_provider_oauth_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kendra_settings`.\n"]
    pub fn set_kendra_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kendra_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_register_settings`.\n"]
    pub fn set_model_register_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_register_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_register_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_settings`.\n"]
    pub fn set_workspace_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CanvasAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CanvasAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettings {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettings {
            direct_deploy_settings: core::default::Default::default(),
            emr_serverless_settings: core::default::Default::default(),
            generative_ai_settings: core::default::Default::default(),
            identity_provider_oauth_settings: core::default::Default::default(),
            kendra_settings: core::default::Default::default(),
            model_register_settings: core::default::Default::default(),
            time_series_forecasting_settings: core::default::Default::default(),
            workspace_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direct_deploy_settings` after provisioning.\n"]
    pub fn direct_deploy_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_DirectDeploySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_deploy_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_serverless_settings` after provisioning.\n"]
    pub fn emr_serverless_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_EmrServerlessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emr_serverless_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `generative_ai_settings` after provisioning.\n"]
    pub fn generative_ai_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_GenerativeAiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_ai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_oauth_settings` after provisioning.\n"]
    pub fn identity_provider_oauth_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider_oauth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_settings` after provisioning.\n"]
    pub fn kendra_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_KendraSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `model_register_settings` after provisioning.\n"]
    pub fn model_register_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_ModelRegisterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_register_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_settings` after provisioning.\n"]
    pub fn workspace_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettings_WorkspaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    built_in_lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.built_in_lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_RSessionAppSettingsDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DefaultUserSettings_RSessionAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettings {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_RSessionAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_RSessionAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettings {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettings {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
    #[doc = "Set the field `access_status`.\n"]
    pub fn set_access_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_group`.\n"]
    pub fn set_user_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
        SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings {
            access_status: core::default::Default::default(),
            user_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_status` after provisioning.\n"]
    pub fn access_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_SharingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_output_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_DefaultUserSettings_SharingSettings {
    #[doc = "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notebook_output_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_output_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_SharingSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SharingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_SharingSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_SharingSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_SharingSettings {
        SagemakerDomain_DefaultUserSettings_SharingSettings {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_SharingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_SharingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_SharingSettingsRef {
        SagemakerDomain_DefaultUserSettings_SharingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_SharingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook_output_option", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_app_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_ml_tools: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
    #[doc = "Set the field `hidden_app_types`.\n"]
    pub fn set_hidden_app_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_app_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_instance_types`.\n"]
    pub fn set_hidden_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_ml_tools`.\n"]
    pub fn set_hidden_ml_tools(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_ml_tools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
        SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings {
            hidden_app_types: core::default::Default::default(),
            hidden_instance_types: core::default::Default::default(),
            hidden_ml_tools: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef {
        SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_app_types` after provisioning.\n"]
    pub fn hidden_app_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_app_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_instance_types` after provisioning.\n"]
    pub fn hidden_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_ml_tools` after provisioning.\n"]
    pub fn hidden_ml_tools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_ml_tools", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsDynamic {
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {}

impl BuildSagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
        SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings {
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef {
        SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DefaultUserSettingsDynamic {
    canvas_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CanvasAppSettings>>,
    code_editor_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings>>,
    custom_file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    custom_posix_user_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    jupyter_lab_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    r_session_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings>>,
    r_studio_server_pro_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings>>,
    sharing_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_SharingSettings>>,
    space_storage_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
    studio_web_portal_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings>>,
    tensor_board_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DefaultUserSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_mount_home_efs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_landing_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    studio_web_portal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canvas_app_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CanvasAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_editor_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_posix_user_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_lab_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_session_app_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_studio_server_pro_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_SharingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_storage_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    studio_web_portal_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tensor_board_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings>>,
    dynamic: SagemakerDomain_DefaultUserSettingsDynamic,
}

impl SagemakerDomain_DefaultUserSettings {
    #[doc = "Set the field `auto_mount_home_efs`.\n"]
    pub fn set_auto_mount_home_efs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auto_mount_home_efs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_landing_uri`.\n"]
    pub fn set_default_landing_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_landing_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `studio_web_portal`.\n"]
    pub fn set_studio_web_portal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.studio_web_portal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CanvasAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canvas_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_editor_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_editor_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r_session_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_app_settings`.\n"]
    pub fn set_r_studio_server_pro_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SharingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sharing_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_SpaceStorageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.space_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `studio_web_portal_settings`.\n"]
    pub fn set_studio_web_portal_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_StudioWebPortalSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.studio_web_portal_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.studio_web_portal_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings_TensorBoardAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DefaultUserSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DefaultUserSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DefaultUserSettings {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DefaultUserSettings {
    pub fn build(self) -> SagemakerDomain_DefaultUserSettings {
        SagemakerDomain_DefaultUserSettings {
            auto_mount_home_efs: core::default::Default::default(),
            default_landing_uri: core::default::Default::default(),
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            studio_web_portal: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            r_studio_server_pro_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            studio_web_portal_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DefaultUserSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DefaultUserSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomain_DefaultUserSettingsRef {
        SagemakerDomain_DefaultUserSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DefaultUserSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_mount_home_efs` after provisioning.\n"]
    pub fn auto_mount_home_efs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_mount_home_efs", self.base))
    }

    #[doc = "Get a reference to the value of field `default_landing_uri` after provisioning.\n"]
    pub fn default_landing_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_landing_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal` after provisioning.\n"]
    pub fn studio_web_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.studio_web_portal", self.base))
    }

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CanvasAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CodeEditorAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_editor_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CustomFileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_CustomPosixUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_posix_user_config", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_JupyterLabAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_JupyterServerAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_KernelGatewayAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_app_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RStudioServerProAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_studio_server_pro_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_SharingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_SpaceStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal_settings` after provisioning.\n"]
    pub fn studio_web_portal_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_StudioWebPortalSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.studio_web_portal_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_TensorBoardAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DomainSettings_DockerSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_docker_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_only_trusted_accounts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerDomain_DomainSettings_DockerSettings {
    #[doc = "Set the field `enable_docker_access`.\n"]
    pub fn set_enable_docker_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enable_docker_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_only_trusted_accounts`.\n"]
    pub fn set_vpc_only_trusted_accounts(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vpc_only_trusted_accounts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DomainSettings_DockerSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DomainSettings_DockerSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DomainSettings_DockerSettings {}

impl BuildSagemakerDomain_DomainSettings_DockerSettings {
    pub fn build(self) -> SagemakerDomain_DomainSettings_DockerSettings {
        SagemakerDomain_DomainSettings_DockerSettings {
            enable_docker_access: core::default::Default::default(),
            vpc_only_trusted_accounts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DomainSettings_DockerSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DomainSettings_DockerSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DomainSettings_DockerSettingsRef {
        SagemakerDomain_DomainSettings_DockerSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DomainSettings_DockerSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_docker_access` after provisioning.\n"]
    pub fn enable_docker_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_docker_access", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_only_trusted_accounts` after provisioning.\n"]
    pub fn vpc_only_trusted_accounts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_only_trusted_accounts", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsDynamic {
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
    domain_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_studio_connect_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_studio_package_manager_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsDynamic,
}

impl SagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
    #[doc = "Set the field `r_studio_connect_url`.\n"]
    pub fn set_r_studio_connect_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.r_studio_connect_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r_studio_package_manager_url`.\n"]
    pub fn set_r_studio_package_manager_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.r_studio_package_manager_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DomainSettings_RStudioServerProDomainSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
    #[doc = ""]
    pub domain_execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
    pub fn build(self) -> SagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
        SagemakerDomain_DomainSettings_RStudioServerProDomainSettings {
            domain_execution_role_arn: self.domain_execution_role_arn,
            r_studio_connect_url: core::default::Default::default(),
            r_studio_package_manager_url: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef {
        SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_execution_role_arn` after provisioning.\n"]
    pub fn domain_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_connect_url` after provisioning.\n"]
    pub fn r_studio_connect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r_studio_connect_url", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_package_manager_url` after provisioning.\n"]
    pub fn r_studio_package_manager_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r_studio_package_manager_url", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DefaultUserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings {
    status: terrars::PrimField<std::string::String>,
}

impl SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings { }

impl terrars::ToListMappable for SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings {
    pub fn build(self) -> SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings {
        SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings { status: self.status }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef {
        SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomain_DomainSettingsDynamic {
    docker_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomain_DomainSettings_DockerSettings>>,
    r_studio_server_pro_domain_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DomainSettings_RStudioServerProDomainSettings>>,
    trusted_identity_propagation_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerDomain_DomainSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_identity_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_settings: std::option::Option<std::vec::Vec<SagemakerDomain_DomainSettings_DockerSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_studio_server_pro_domain_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DomainSettings_RStudioServerProDomainSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_identity_propagation_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings>>,
    dynamic: SagemakerDomain_DomainSettingsDynamic,
}

impl SagemakerDomain_DomainSettings {
    #[doc = "Set the field `execution_role_identity_config`.\n"]
    pub fn set_execution_role_identity_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_identity_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_settings`.\n"]
    pub fn set_docker_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomain_DomainSettings_DockerSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.docker_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_domain_settings`.\n"]
    pub fn set_r_studio_server_pro_domain_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomain_DomainSettings_RStudioServerProDomainSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_domain_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_domain_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trusted_identity_propagation_settings`.\n"]
    pub fn set_trusted_identity_propagation_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trusted_identity_propagation_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trusted_identity_propagation_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_DomainSettings {
    type O = terrars::BlockAssignable<SagemakerDomain_DomainSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_DomainSettings {}

impl BuildSagemakerDomain_DomainSettings {
    pub fn build(self) -> SagemakerDomain_DomainSettings {
        SagemakerDomain_DomainSettings {
            execution_role_identity_config: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            docker_settings: core::default::Default::default(),
            r_studio_server_pro_domain_settings: core::default::Default::default(),
            trusted_identity_propagation_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_DomainSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_DomainSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomain_DomainSettingsRef {
        SagemakerDomain_DomainSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_DomainSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_identity_config` after provisioning.\n"]
    pub fn execution_role_identity_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_identity_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_settings` after provisioning.\n"]
    pub fn docker_settings(&self) -> terrars::ListRef<SagemakerDomain_DomainSettings_DockerSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_domain_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_domain_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DomainSettings_RStudioServerProDomainSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_studio_server_pro_domain_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_identity_propagation_settings` after provisioning.\n"]
    pub fn trusted_identity_propagation_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomain_DomainSettings_TrustedIdentityPropagationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_identity_propagation_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomain_RetentionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_efs_file_system: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomain_RetentionPolicy {
    #[doc = "Set the field `home_efs_file_system`.\n"]
    pub fn set_home_efs_file_system(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.home_efs_file_system = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomain_RetentionPolicy {
    type O = terrars::BlockAssignable<SagemakerDomain_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomain_RetentionPolicy {}

impl BuildSagemakerDomain_RetentionPolicy {
    pub fn build(self) -> SagemakerDomain_RetentionPolicy {
        SagemakerDomain_RetentionPolicy { home_efs_file_system: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerDomain_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomain_RetentionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomain_RetentionPolicyRef {
        SagemakerDomain_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomain_RetentionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system` after provisioning.\n"]
    pub fn home_efs_file_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system", self.base))
    }
}
