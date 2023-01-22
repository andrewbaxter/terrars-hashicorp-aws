use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_network_access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_security_group_management: Option<PrimField<String>>,
    auth_mode: PrimField<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_space_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_user_settings: Option<Vec<SagemakerDomainDefaultUserSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_settings: Option<Vec<SagemakerDomainDomainSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy: Option<Vec<SagemakerDomainRetentionPolicyEl>>,
    dynamic: SagemakerDomainDynamic,
}

struct SagemakerDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerDomainData>,
}

#[derive(Clone)]
pub struct SagemakerDomain(Rc<SagemakerDomain_>);

impl SagemakerDomain {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `app_network_access_type`.\n"]
    pub fn set_app_network_access_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_network_access_type = Some(v.into());
        self
    }

    #[doc= "Set the field `app_security_group_management`.\n"]
    pub fn set_app_security_group_management(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_security_group_management = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `default_space_settings`.\n"]
    pub fn set_default_space_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_space_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_space_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_user_settings`.\n"]
    pub fn set_default_user_settings(self, v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_user_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_user_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `domain_settings`.\n"]
    pub fn set_domain_settings(self, v: impl Into<BlockAssignable<SagemakerDomainDomainSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(self, v: impl Into<BlockAssignable<SagemakerDomainRetentionPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_security_group_management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id_for_domain_boundary", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> ListRef<SagemakerDomainDomainSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<SagemakerDomainRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

impl Resource for SagemakerDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for SagemakerDomain {
    type O = ListRef<SagemakerDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDomain {
    pub tf_id: String,
    #[doc= ""]
    pub auth_mode: PrimField<String>,
    #[doc= ""]
    pub domain_name: PrimField<String>,
    #[doc= ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc= ""]
    pub vpc_id: PrimField<String>,
}

impl BuildSagemakerDomain {
    pub fn build(self, stack: &mut Stack) -> SagemakerDomain {
        let out = SagemakerDomain(Rc::new(SagemakerDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_security_group_management", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id_for_domain_boundary", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> ListRef<SagemakerDomainDomainSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<SagemakerDomainRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc= ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElDynamic {
    jupyter_server_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>,
    >,
    kernel_gateway_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsEl {
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsEl {
    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsEl {
    #[doc= ""]
    pub execution_role: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsEl {
        SagemakerDomainDefaultSpaceSettingsEl {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_forecast_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[doc= "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_forecast_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_forecast_role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic {
    time_series_forecasting_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    time_series_forecasting_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    #[doc= "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc= ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_output_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_path: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    #[doc= "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notebook_output_option = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_output_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notebook_output_option", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElDynamic {
    canvas_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    jupyter_server_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>,
    >,
    kernel_gateway_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>,
    >,
    r_session_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    sharing_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    tensor_board_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsEl {
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_session_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tensor_board_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsEl {
    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canvas_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_session_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sharing_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsEl {
    #[doc= ""]
    pub execution_role: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElSharingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_identity_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
}

impl SagemakerDomainDomainSettingsEl {
    #[doc= "Set the field `execution_role_identity_config`.\n"]
    pub fn set_execution_role_identity_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_identity_config = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDomainSettingsEl {
    type O = BlockAssignable<SagemakerDomainDomainSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDomainSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDomainSettingsElRef {
        SagemakerDomainDomainSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `execution_role_identity_config` after provisioning.\n"]
    pub fn execution_role_identity_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_identity_config", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    home_efs_file_system: Option<PrimField<String>>,
}

impl SagemakerDomainRetentionPolicyEl {
    #[doc= "Set the field `home_efs_file_system`.\n"]
    pub fn set_home_efs_file_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.home_efs_file_system = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainRetentionPolicyEl {
    type O = BlockAssignable<SagemakerDomainRetentionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainRetentionPolicyElRef {
        SagemakerDomainRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system` after provisioning.\n"]
    pub fn home_efs_file_system(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDynamic {
    default_space_settings: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsEl>>,
    default_user_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsEl>>,
    domain_settings: Option<DynamicBlock<SagemakerDomainDomainSettingsEl>>,
    retention_policy: Option<DynamicBlock<SagemakerDomainRetentionPolicyEl>>,
}
