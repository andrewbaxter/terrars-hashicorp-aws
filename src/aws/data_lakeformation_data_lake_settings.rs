#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLakeformationDataLakeSettingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLakeformationDataLakeSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLakeformationDataLakeSettingsData_>,
}

#[derive(Clone)]
pub struct DataLakeformationDataLakeSettings(std::rc::Rc<DataLakeformationDataLakeSettings_>);

impl DataLakeformationDataLakeSettings {
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

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `admins` after provisioning.\n"]
    pub fn admins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admins", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_external_data_filtering` after provisioning.\n"]
    pub fn allow_external_data_filtering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_external_data_filtering", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorized_session_tag_value_list` after provisioning.\n"]
    pub fn authorized_session_tag_value_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_session_tag_value_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> terrars::ListRef<DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `external_data_filtering_allow_list` after provisioning.\n"]
    pub fn external_data_filtering_allow_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.external_data_filtering_allow_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only_admins` after provisioning.\n"]
    pub fn read_only_admins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.read_only_admins", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_resource_owners` after provisioning.\n"]
    pub fn trusted_resource_owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_resource_owners", self.extract_ref()))
    }
}

impl terrars::Referable for DataLakeformationDataLakeSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLakeformationDataLakeSettings { }

impl terrars::ToListMappable for DataLakeformationDataLakeSettings {
    type O = terrars::ListRef<DataLakeformationDataLakeSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLakeformationDataLakeSettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lakeformation_data_lake_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLakeformationDataLakeSettings {
    pub tf_id: std::string::String,
}

impl BuildDataLakeformationDataLakeSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLakeformationDataLakeSettings {
        let out = DataLakeformationDataLakeSettings(std::rc::Rc::new(DataLakeformationDataLakeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLakeformationDataLakeSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLakeformationDataLakeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationDataLakeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLakeformationDataLakeSettingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `admins` after provisioning.\n"]
    pub fn admins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admins", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_external_data_filtering` after provisioning.\n"]
    pub fn allow_external_data_filtering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_external_data_filtering", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorized_session_tag_value_list` after provisioning.\n"]
    pub fn authorized_session_tag_value_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_session_tag_value_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> terrars::ListRef<DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `external_data_filtering_allow_list` after provisioning.\n"]
    pub fn external_data_filtering_allow_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.external_data_filtering_allow_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only_admins` after provisioning.\n"]
    pub fn read_only_admins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.read_only_admins", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_resource_owners` after provisioning.\n"]
    pub fn trusted_resource_owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_resource_owners", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    type O = terrars::BlockAssignable<DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {}

impl BuildDataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    pub fn build(self) -> DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
        DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
        DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
    type O = terrars::BlockAssignable<DataLakeformationDataLakeSettings_CreateTableDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationDataLakeSettings_CreateTableDefaultPermissions {}

impl BuildDataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
    pub fn build(self) -> DataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
        DataLakeformationDataLakeSettings_CreateTableDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
        DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
