use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDomainData {
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
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_space_settings: std::option::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_user_settings: std::option::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_settings: std::option::Option<std::vec::Vec<SagemakerDomainDomainSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<SagemakerDomainRetentionPolicyEl>>,
    dynamic: SagemakerDomainDynamic,
}

struct SagemakerDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerDomainData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomainDomainSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomainRetentionPolicyEl>>,
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

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id_for_domain_boundary", self.extract_ref()),
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
    pub fn default_space_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> terrars::ListRef<SagemakerDomainDomainSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<SagemakerDomainRetentionPolicyElRef> {
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
            data: std::cell::RefCell::new(SagemakerDomainData {
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
                subnet_ids: self.subnet_ids,
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

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id_for_domain_boundary", self.extract_ref()),
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
    pub fn default_space_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> terrars::ListRef<SagemakerDomainDomainSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<SagemakerDomainRetentionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O =
        terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
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
    ) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
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
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
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
                        terrars
                        ::BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl,
                        >,
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
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
    ) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElDynamic {
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsEl {
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
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
}

impl terrars::ToListMappable for SagemakerDomainDefaultSpaceSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsEl {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsEl {
        SagemakerDomainDefaultSpaceSettingsEl {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultSpaceSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomainDefaultSpaceSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElRef {
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

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_forecast_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
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

impl terrars
::ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
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

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic {
    time_series_forecasting_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_series_forecasting_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
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
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
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
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
            time_series_forecasting_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O =
        terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
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
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
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
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
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
                        terrars
                        ::BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>,
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
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
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
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
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>,
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
                        ::BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_output_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElSharingSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
        SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
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
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElDynamic {
    canvas_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    r_session_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    sharing_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    tensor_board_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsEl {
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canvas_app_settings: std::option::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_session_app_settings: std::option::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing_settings: std::option::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tensor_board_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
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

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
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

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
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

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
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

impl terrars::ToListMappable for SagemakerDomainDefaultUserSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDefaultUserSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsEl {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDomainDefaultUserSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsEl {
        SagemakerDomainDefaultUserSettingsEl {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDefaultUserSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomainDefaultUserSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRef {
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

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElSharingSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_identity_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerDomainDomainSettingsEl {
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
}

impl terrars::ToListMappable for SagemakerDomainDomainSettingsEl {
    type O = terrars::BlockAssignable<SagemakerDomainDomainSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDomainSettingsEl {}

impl BuildSagemakerDomainDomainSettingsEl {
    pub fn build(self) -> SagemakerDomainDomainSettingsEl {
        SagemakerDomainDomainSettingsEl {
            execution_role_identity_config: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDomainSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainDomainSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomainDomainSettingsElRef {
        SagemakerDomainDomainSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElRef {
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
}

#[derive(Serialize)]
pub struct SagemakerDomainRetentionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    home_efs_file_system: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDomainRetentionPolicyEl {
    #[doc = "Set the field `home_efs_file_system`.\n"]
    pub fn set_home_efs_file_system(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.home_efs_file_system = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDomainRetentionPolicyEl {
    type O = terrars::BlockAssignable<SagemakerDomainRetentionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainRetentionPolicyEl {}

impl BuildSagemakerDomainRetentionPolicyEl {
    pub fn build(self) -> SagemakerDomainRetentionPolicyEl {
        SagemakerDomainRetentionPolicyEl { home_efs_file_system: core::default::Default::default() }
    }
}

pub struct SagemakerDomainRetentionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDomainRetentionPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerDomainRetentionPolicyElRef {
        SagemakerDomainRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainRetentionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system` after provisioning.\n"]
    pub fn home_efs_file_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDynamic {
    default_space_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomainDefaultSpaceSettingsEl>>,
    default_user_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomainDefaultUserSettingsEl>>,
    domain_settings: std::option::Option<terrars::DynamicBlock<SagemakerDomainDomainSettingsEl>>,
    retention_policy: std::option::Option<terrars::DynamicBlock<SagemakerDomainRetentionPolicyEl>>,
}
