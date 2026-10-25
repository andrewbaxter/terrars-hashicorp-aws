#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataWorkspacesDirectoryData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    directory_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataWorkspacesDirectory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkspacesDirectoryData_>,
}

#[derive(Clone)]
pub struct DataWorkspacesDirectory(std::rc::Rc<DataWorkspacesDirectory_>);

impl DataWorkspacesDirectory {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> terrars::SetRef<DataWorkspacesDirectory_ActiveDirectoryConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.active_directory_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> terrars::ListRef<DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> terrars::ListRef<DataWorkspacesDirectory_SamlPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> terrars::ListRef<DataWorkspacesDirectory_SelfServicePermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> terrars::ListRef<DataWorkspacesDirectory_WorkspaceAccessPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(
        &self,
    ) -> terrars::ListRef<DataWorkspacesDirectory_WorkspaceCreationPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkspacesDirectory {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkspacesDirectory { }

impl terrars::ToListMappable for DataWorkspacesDirectory {
    type O = terrars::ListRef<DataWorkspacesDirectoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkspacesDirectory_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_workspaces_directory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkspacesDirectory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
}

impl BuildDataWorkspacesDirectory {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkspacesDirectory {
        let out = DataWorkspacesDirectory(std::rc::Rc::new(DataWorkspacesDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkspacesDirectoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkspacesDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkspacesDirectoryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> terrars::SetRef<DataWorkspacesDirectory_ActiveDirectoryConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.active_directory_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> terrars::ListRef<DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> terrars::ListRef<DataWorkspacesDirectory_SamlPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> terrars::ListRef<DataWorkspacesDirectory_SelfServicePermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> terrars::ListRef<DataWorkspacesDirectory_WorkspaceAccessPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(
        &self,
    ) -> terrars::ListRef<DataWorkspacesDirectory_WorkspaceCreationPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_ActiveDirectoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesDirectory_ActiveDirectoryConfig {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_secret_arn`.\n"]
    pub fn set_service_account_secret_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_ActiveDirectoryConfig {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_ActiveDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_ActiveDirectoryConfig {}

impl BuildDataWorkspacesDirectory_ActiveDirectoryConfig {
    pub fn build(self) -> DataWorkspacesDirectory_ActiveDirectoryConfig {
        DataWorkspacesDirectory_ActiveDirectoryConfig {
            domain_name: core::default::Default::default(),
            service_account_secret_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_ActiveDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_ActiveDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_ActiveDirectoryConfigRef {
        DataWorkspacesDirectory_ActiveDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_ActiveDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_secret_arn` after provisioning.\n"]
    pub fn service_account_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_CertificateBasedAuthProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesDirectory_CertificateBasedAuthProperties {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_CertificateBasedAuthProperties {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_CertificateBasedAuthProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_CertificateBasedAuthProperties {}

impl BuildDataWorkspacesDirectory_CertificateBasedAuthProperties {
    pub fn build(self) -> DataWorkspacesDirectory_CertificateBasedAuthProperties {
        DataWorkspacesDirectory_CertificateBasedAuthProperties {
            certificate_authority_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef {
        DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_CertificateBasedAuthPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_SamlProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relay_state_parameter_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_access_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesDirectory_SamlProperties {
    #[doc = "Set the field `relay_state_parameter_name`.\n"]
    pub fn set_relay_state_parameter_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.relay_state_parameter_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_access_url`.\n"]
    pub fn set_user_access_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_access_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_SamlProperties {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_SamlProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_SamlProperties {}

impl BuildDataWorkspacesDirectory_SamlProperties {
    pub fn build(self) -> DataWorkspacesDirectory_SamlProperties {
        DataWorkspacesDirectory_SamlProperties {
            relay_state_parameter_name: core::default::Default::default(),
            status: core::default::Default::default(),
            user_access_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_SamlPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_SamlPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkspacesDirectory_SamlPropertiesRef {
        DataWorkspacesDirectory_SamlPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_SamlPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `relay_state_parameter_name` after provisioning.\n"]
    pub fn relay_state_parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay_state_parameter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_access_url` after provisioning.\n"]
    pub fn user_access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_access_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_SelfServicePermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    change_compute_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    increase_volume_size: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rebuild_workspace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_workspace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_running_mode: std::option::Option<terrars::PrimField<bool>>,
}

impl DataWorkspacesDirectory_SelfServicePermissions {
    #[doc = "Set the field `change_compute_type`.\n"]
    pub fn set_change_compute_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.change_compute_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `increase_volume_size`.\n"]
    pub fn set_increase_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.increase_volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rebuild_workspace`.\n"]
    pub fn set_rebuild_workspace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rebuild_workspace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_workspace`.\n"]
    pub fn set_restart_workspace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.restart_workspace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `switch_running_mode`.\n"]
    pub fn set_switch_running_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.switch_running_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_SelfServicePermissions {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_SelfServicePermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_SelfServicePermissions {}

impl BuildDataWorkspacesDirectory_SelfServicePermissions {
    pub fn build(self) -> DataWorkspacesDirectory_SelfServicePermissions {
        DataWorkspacesDirectory_SelfServicePermissions {
            change_compute_type: core::default::Default::default(),
            increase_volume_size: core::default::Default::default(),
            rebuild_workspace: core::default::Default::default(),
            restart_workspace: core::default::Default::default(),
            switch_running_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_SelfServicePermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_SelfServicePermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_SelfServicePermissionsRef {
        DataWorkspacesDirectory_SelfServicePermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_SelfServicePermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `change_compute_type` after provisioning.\n"]
    pub fn change_compute_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.change_compute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `increase_volume_size` after provisioning.\n"]
    pub fn increase_volume_size(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increase_volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rebuild_workspace` after provisioning.\n"]
    pub fn rebuild_workspace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rebuild_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_workspace` after provisioning.\n"]
    pub fn restart_workspace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `switch_running_mode` after provisioning.\n"]
    pub fn switch_running_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_running_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
    #[doc = "Set the field `access_endpoint_type`.\n"]
    pub fn set_access_endpoint_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_endpoint_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_endpoint_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
    type O =
        terrars
        ::BlockAssignable<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {}

impl BuildDataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
    pub fn build(self) -> DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
        DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints {
            access_endpoint_type: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef {
        DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_endpoint_type` after provisioning.\n"]
    pub fn access_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_endpoint_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_endpoints: std
    ::option
    ::Option<
        terrars::SetField<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_fallback_protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
    #[doc = "Set the field `access_endpoints`.\n"]
    pub fn set_access_endpoints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpoints,
                        >,
                    >,
    ) -> Self {
        self.access_endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_fallback_protocols`.\n"]
    pub fn set_internet_fallback_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.internet_fallback_protocols = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {}

impl BuildDataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
    pub fn build(self) -> DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
        DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig {
            access_endpoints: core::default::Default::default(),
            internet_fallback_protocols: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef {
        DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_endpoints` after provisioning.\n"]
    pub fn access_endpoints(
        &self,
    ) -> terrars::SetRef<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig_AccessEndpointsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_fallback_protocols` after provisioning.\n"]
    pub fn internet_fallback_protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internet_fallback_protocols", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_WorkspaceAccessProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_endpoint_config: std
    ::option
    ::Option<terrars::ListField<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_android: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_chromeos: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_ios: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_linux: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_osx: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_web: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_windows: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type_zeroclient: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesDirectory_WorkspaceAccessProperties {
    #[doc = "Set the field `access_endpoint_config`.\n"]
    pub fn set_access_endpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfig>,
                    >,
    ) -> Self {
        self.access_endpoint_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_android`.\n"]
    pub fn set_device_type_android(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_android = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_chromeos`.\n"]
    pub fn set_device_type_chromeos(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_chromeos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_ios`.\n"]
    pub fn set_device_type_ios(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_ios = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_linux`.\n"]
    pub fn set_device_type_linux(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_linux = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_osx`.\n"]
    pub fn set_device_type_osx(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_osx = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_web`.\n"]
    pub fn set_device_type_web(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type_web = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_windows`.\n"]
    pub fn set_device_type_windows(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_windows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type_zeroclient`.\n"]
    pub fn set_device_type_zeroclient(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_type_zeroclient = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_WorkspaceAccessProperties {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_WorkspaceAccessProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_WorkspaceAccessProperties {}

impl BuildDataWorkspacesDirectory_WorkspaceAccessProperties {
    pub fn build(self) -> DataWorkspacesDirectory_WorkspaceAccessProperties {
        DataWorkspacesDirectory_WorkspaceAccessProperties {
            access_endpoint_config: core::default::Default::default(),
            device_type_android: core::default::Default::default(),
            device_type_chromeos: core::default::Default::default(),
            device_type_ios: core::default::Default::default(),
            device_type_linux: core::default::Default::default(),
            device_type_osx: core::default::Default::default(),
            device_type_web: core::default::Default::default(),
            device_type_windows: core::default::Default::default(),
            device_type_zeroclient: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_WorkspaceAccessPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_WorkspaceAccessPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_WorkspaceAccessPropertiesRef {
        DataWorkspacesDirectory_WorkspaceAccessPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_WorkspaceAccessPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_endpoint_config` after provisioning.\n"]
    pub fn access_endpoint_config(
        &self,
    ) -> terrars::ListRef<DataWorkspacesDirectory_WorkspaceAccessProperties_AccessEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_android` after provisioning.\n"]
    pub fn device_type_android(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_android", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_chromeos` after provisioning.\n"]
    pub fn device_type_chromeos(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_chromeos", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_ios` after provisioning.\n"]
    pub fn device_type_ios(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_ios", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_linux` after provisioning.\n"]
    pub fn device_type_linux(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_linux", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_osx` after provisioning.\n"]
    pub fn device_type_osx(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_osx", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_web` after provisioning.\n"]
    pub fn device_type_web(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_web", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_windows` after provisioning.\n"]
    pub fn device_type_windows(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_windows", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_zeroclient` after provisioning.\n"]
    pub fn device_type_zeroclient(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type_zeroclient", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectory_WorkspaceCreationProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ou: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_internet_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_maintenance_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_enabled_as_local_administrator: std::option::Option<terrars::PrimField<bool>>,
}

impl DataWorkspacesDirectory_WorkspaceCreationProperties {
    #[doc = "Set the field `custom_security_group_id`.\n"]
    pub fn set_custom_security_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_security_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_ou`.\n"]
    pub fn set_default_ou(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_ou = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_internet_access`.\n"]
    pub fn set_enable_internet_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_internet_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_maintenance_mode`.\n"]
    pub fn set_enable_maintenance_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_maintenance_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_enabled_as_local_administrator`.\n"]
    pub fn set_user_enabled_as_local_administrator(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.user_enabled_as_local_administrator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesDirectory_WorkspaceCreationProperties {
    type O = terrars::BlockAssignable<DataWorkspacesDirectory_WorkspaceCreationProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectory_WorkspaceCreationProperties {}

impl BuildDataWorkspacesDirectory_WorkspaceCreationProperties {
    pub fn build(self) -> DataWorkspacesDirectory_WorkspaceCreationProperties {
        DataWorkspacesDirectory_WorkspaceCreationProperties {
            custom_security_group_id: core::default::Default::default(),
            default_ou: core::default::Default::default(),
            enable_internet_access: core::default::Default::default(),
            enable_maintenance_mode: core::default::Default::default(),
            user_enabled_as_local_administrator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesDirectory_WorkspaceCreationPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesDirectory_WorkspaceCreationPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkspacesDirectory_WorkspaceCreationPropertiesRef {
        DataWorkspacesDirectory_WorkspaceCreationPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectory_WorkspaceCreationPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_security_group_id` after provisioning.\n"]
    pub fn custom_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ou` after provisioning.\n"]
    pub fn default_ou(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ou", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\n"]
    pub fn enable_internet_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_maintenance_mode` after provisioning.\n"]
    pub fn enable_maintenance_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_maintenance_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `user_enabled_as_local_administrator` after provisioning.\n"]
    pub fn user_enabled_as_local_administrator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_enabled_as_local_administrator", self.base))
    }
}
