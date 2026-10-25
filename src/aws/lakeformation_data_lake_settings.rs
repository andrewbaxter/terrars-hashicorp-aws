#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationDataLakeSettingsData_ {
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
    admins: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_external_data_filtering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_full_table_external_data_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_session_tag_value_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_data_filtering_allow_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only_admins: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_resource_owners: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_database_default_permissions: std
    ::option
    ::Option<std::vec::Vec<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_table_default_permissions: std
    ::option
    ::Option<std::vec::Vec<LakeformationDataLakeSettings_CreateTableDefaultPermissions>>,
    dynamic: LakeformationDataLakeSettingsDynamic,
}

struct LakeformationDataLakeSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LakeformationDataLakeSettingsData_>,
}

#[derive(Clone)]
pub struct LakeformationDataLakeSettings(std::rc::Rc<LakeformationDataLakeSettings_>);

impl LakeformationDataLakeSettings {
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

    #[doc = "Set the field `admins`.\n"]
    pub fn set_admins(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().admins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_external_data_filtering`.\n"]
    pub fn set_allow_external_data_filtering(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_external_data_filtering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_full_table_external_data_access`.\n"]
    pub fn set_allow_full_table_external_data_access(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().allow_full_table_external_data_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorized_session_tag_value_list`.\n"]
    pub fn set_authorized_session_tag_value_list(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().authorized_session_tag_value_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_data_filtering_allow_list`.\n"]
    pub fn set_external_data_filtering_allow_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().external_data_filtering_allow_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only_admins`.\n"]
    pub fn set_read_only_admins(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().read_only_admins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_resource_owners`.\n"]
    pub fn set_trusted_resource_owners(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().trusted_resource_owners = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_database_default_permissions`.\n"]
    pub fn set_create_database_default_permissions(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_database_default_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.create_database_default_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_table_default_permissions`.\n"]
    pub fn set_create_table_default_permissions(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LakeformationDataLakeSettings_CreateTableDefaultPermissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_table_default_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.create_table_default_permissions = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> terrars::ListRef<LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }
}

impl terrars::Referable for LakeformationDataLakeSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LakeformationDataLakeSettings { }

impl terrars::ToListMappable for LakeformationDataLakeSettings {
    type O = terrars::ListRef<LakeformationDataLakeSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LakeformationDataLakeSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lakeformation_data_lake_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationDataLakeSettings {
    pub tf_id: std::string::String,
}

impl BuildLakeformationDataLakeSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> LakeformationDataLakeSettings {
        let out = LakeformationDataLakeSettings(std::rc::Rc::new(LakeformationDataLakeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LakeformationDataLakeSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admins: core::default::Default::default(),
                allow_external_data_filtering: core::default::Default::default(),
                allow_full_table_external_data_access: core::default::Default::default(),
                authorized_session_tag_value_list: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                external_data_filtering_allow_list: core::default::Default::default(),
                id: core::default::Default::default(),
                parameters: core::default::Default::default(),
                read_only_admins: core::default::Default::default(),
                region: core::default::Default::default(),
                trusted_resource_owners: core::default::Default::default(),
                create_database_default_permissions: core::default::Default::default(),
                create_table_default_permissions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationDataLakeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataLakeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LakeformationDataLakeSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> terrars::ListRef<LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataLakeSettingsDynamic {
    create_database_default_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions>>,
    create_table_default_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationDataLakeSettings_CreateTableDefaultPermissions>>,
}

#[derive(Serialize)]
pub struct LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
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

impl terrars::ToListMappable for LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    type O = terrars::BlockAssignable<LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {}

impl BuildLakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
    pub fn build(self) -> LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
        LakeformationDataLakeSettings_CreateDatabaseDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
        LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataLakeSettings_CreateDatabaseDefaultPermissionsRef {
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
pub struct LakeformationDataLakeSettings_CreateTableDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationDataLakeSettings_CreateTableDefaultPermissions {
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

impl terrars::ToListMappable for LakeformationDataLakeSettings_CreateTableDefaultPermissions {
    type O = terrars::BlockAssignable<LakeformationDataLakeSettings_CreateTableDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataLakeSettings_CreateTableDefaultPermissions {}

impl BuildLakeformationDataLakeSettings_CreateTableDefaultPermissions {
    pub fn build(self) -> LakeformationDataLakeSettings_CreateTableDefaultPermissions {
        LakeformationDataLakeSettings_CreateTableDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
        LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataLakeSettings_CreateTableDefaultPermissionsRef {
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
