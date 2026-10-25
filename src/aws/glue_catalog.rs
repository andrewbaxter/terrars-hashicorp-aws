#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogData_ {
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
    allow_full_table_external_data_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_child_resource_permissions_with_default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_properties: std::option::Option<std::vec::Vec<GlueCatalog_CatalogProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_database_default_permissions: std::option::Option<std::vec::Vec<GlueCatalog_CreateDatabaseDefaultPermissions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_table_default_permissions: std::option::Option<std::vec::Vec<GlueCatalog_CreateTableDefaultPermissions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_catalog: std::option::Option<std::vec::Vec<GlueCatalog_FederatedCatalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_redshift_catalog: std::option::Option<std::vec::Vec<GlueCatalog_TargetRedshiftCatalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GlueCatalog_Timeouts>,
    dynamic: GlueCatalogDynamic,
}

struct GlueCatalog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCatalogData_>,
}

#[derive(Clone)]
pub struct GlueCatalog(std::rc::Rc<GlueCatalog_>);

impl GlueCatalog {
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

    #[doc = "Set the field `allow_full_table_external_data_access`.\n"]
    pub fn set_allow_full_table_external_data_access(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().allow_full_table_external_data_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_child_resource_permissions_with_default`.\n"]
    pub fn set_overwrite_child_resource_permissions_with_default(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().overwrite_child_resource_permissions_with_default =
            std::option::Option::Some(v.into());
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

    #[doc = "Set the field `catalog_properties`.\n"]
    pub fn set_catalog_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CatalogProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().catalog_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.catalog_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_database_default_permissions`.\n"]
    pub fn set_create_database_default_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CreateDatabaseDefaultPermissions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CreateTableDefaultPermissions>>,
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

    #[doc = "Set the field `federated_catalog`.\n"]
    pub fn set_federated_catalog(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_FederatedCatalog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().federated_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.federated_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_redshift_catalog`.\n"]
    pub fn set_target_redshift_catalog(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_TargetRedshiftCatalog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_redshift_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_redshift_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GlueCatalog_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `overwrite_child_resource_permissions_with_default` after provisioning.\n"]
    pub fn overwrite_child_resource_permissions_with_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_child_resource_permissions_with_default", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_properties` after provisioning.\n"]
    pub fn catalog_properties(&self) -> terrars::ListRef<GlueCatalog_CatalogPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<GlueCatalog_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(&self) -> terrars::ListRef<GlueCatalog_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `federated_catalog` after provisioning.\n"]
    pub fn federated_catalog(&self) -> terrars::ListRef<GlueCatalog_FederatedCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_redshift_catalog` after provisioning.\n"]
    pub fn target_redshift_catalog(&self) -> terrars::ListRef<GlueCatalog_TargetRedshiftCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_redshift_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueCatalog_TimeoutsRef {
        GlueCatalog_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GlueCatalog {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueCatalog { }

impl terrars::ToListMappable for GlueCatalog {
    type O = terrars::ListRef<GlueCatalogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueCatalog_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_catalog".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalog {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalog {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueCatalog {
        let out = GlueCatalog(std::rc::Rc::new(GlueCatalog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueCatalogData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_full_table_external_data_access: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                overwrite_child_resource_permissions_with_default: core::default::Default::default(),
                parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                catalog_properties: core::default::Default::default(),
                create_database_default_permissions: core::default::Default::default(),
                create_table_default_permissions: core::default::Default::default(),
                federated_catalog: core::default::Default::default(),
                target_redshift_catalog: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueCatalogRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `overwrite_child_resource_permissions_with_default` after provisioning.\n"]
    pub fn overwrite_child_resource_permissions_with_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite_child_resource_permissions_with_default", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_properties` after provisioning.\n"]
    pub fn catalog_properties(&self) -> terrars::ListRef<GlueCatalog_CatalogPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<GlueCatalog_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(&self) -> terrars::ListRef<GlueCatalog_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `federated_catalog` after provisioning.\n"]
    pub fn federated_catalog(&self) -> terrars::ListRef<GlueCatalog_FederatedCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_redshift_catalog` after provisioning.\n"]
    pub fn target_redshift_catalog(&self) -> terrars::ListRef<GlueCatalog_TargetRedshiftCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_redshift_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueCatalog_TimeoutsRef {
        GlueCatalog_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogDynamic {
    catalog_properties: std::option::Option<terrars::DynamicBlock<GlueCatalog_CatalogProperties>>,
    create_database_default_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalog_CreateDatabaseDefaultPermissions>>,
    create_table_default_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalog_CreateTableDefaultPermissions>>,
    federated_catalog: std::option::Option<terrars::DynamicBlock<GlueCatalog_FederatedCatalog>>,
    target_redshift_catalog: std::option::Option<terrars::DynamicBlock<GlueCatalog_TargetRedshiftCatalog>>,
}

#[derive(Serialize)]
pub struct GlueCatalog_CatalogProperties_DataLakeAccessProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transfer_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalog_CatalogProperties_DataLakeAccessProperties {
    #[doc = "Set the field `catalog_type`.\n"]
    pub fn set_catalog_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_lake_access`.\n"]
    pub fn set_data_lake_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_lake_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_transfer_role`.\n"]
    pub fn set_data_transfer_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_transfer_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CatalogProperties_DataLakeAccessProperties {
    type O = terrars::BlockAssignable<GlueCatalog_CatalogProperties_DataLakeAccessProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CatalogProperties_DataLakeAccessProperties {}

impl BuildGlueCatalog_CatalogProperties_DataLakeAccessProperties {
    pub fn build(self) -> GlueCatalog_CatalogProperties_DataLakeAccessProperties {
        GlueCatalog_CatalogProperties_DataLakeAccessProperties {
            catalog_type: core::default::Default::default(),
            data_lake_access: core::default::Default::default(),
            data_transfer_role: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
        GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_type` after provisioning.\n"]
    pub fn catalog_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_lake_access` after provisioning.\n"]
    pub fn data_lake_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_access", self.base))
    }

    #[doc = "Get a reference to the value of field `data_transfer_role` after provisioning.\n"]
    pub fn data_transfer_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_role", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_workgroup_name` after provisioning.\n"]
    pub fn managed_workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_workgroup_name", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_workgroup_status` after provisioning.\n"]
    pub fn managed_workgroup_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_workgroup_status", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_database_name` after provisioning.\n"]
    pub fn redshift_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redshift_database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compaction: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orphan_file_deletion: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    #[doc = "Set the field `compaction`.\n"]
    pub fn set_compaction(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orphan_file_deletion`.\n"]
    pub fn set_orphan_file_deletion(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.orphan_file_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    type O = terrars::BlockAssignable<GlueCatalog_CatalogProperties_IcebergOptimizationProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CatalogProperties_IcebergOptimizationProperties {}

impl BuildGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    pub fn build(self) -> GlueCatalog_CatalogProperties_IcebergOptimizationProperties {
        GlueCatalog_CatalogProperties_IcebergOptimizationProperties {
            compaction: core::default::Default::default(),
            orphan_file_deletion: core::default::Default::default(),
            retention: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
        GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compaction` after provisioning.\n"]
    pub fn compaction(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.compaction", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_deletion` after provisioning.\n"]
    pub fn orphan_file_deletion(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.orphan_file_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.retention", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalog_CatalogPropertiesDynamic {
    data_lake_access_properties: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalog_CatalogProperties_DataLakeAccessProperties>>,
    iceberg_optimization_properties: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalog_CatalogProperties_IcebergOptimizationProperties>>,
}

#[derive(Serialize)]
pub struct GlueCatalog_CatalogProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_access_properties: std
    ::option
    ::Option<std::vec::Vec<GlueCatalog_CatalogProperties_DataLakeAccessProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_optimization_properties: std
    ::option
    ::Option<std::vec::Vec<GlueCatalog_CatalogProperties_IcebergOptimizationProperties>>,
    dynamic: GlueCatalog_CatalogPropertiesDynamic,
}

impl GlueCatalog_CatalogProperties {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_lake_access_properties`.\n"]
    pub fn set_data_lake_access_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CatalogProperties_DataLakeAccessProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_lake_access_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_lake_access_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iceberg_optimization_properties`.\n"]
    pub fn set_iceberg_optimization_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueCatalog_CatalogProperties_IcebergOptimizationProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_optimization_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_optimization_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CatalogProperties {
    type O = terrars::BlockAssignable<GlueCatalog_CatalogProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CatalogProperties {}

impl BuildGlueCatalog_CatalogProperties {
    pub fn build(self) -> GlueCatalog_CatalogProperties {
        GlueCatalog_CatalogProperties {
            custom_properties: core::default::Default::default(),
            data_lake_access_properties: core::default::Default::default(),
            iceberg_optimization_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CatalogPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CatalogPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalog_CatalogPropertiesRef {
        GlueCatalog_CatalogPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CatalogPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `data_lake_access_properties` after provisioning.\n"]
    pub fn data_lake_access_properties(
        &self,
    ) -> terrars::ListRef<GlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_lake_access_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `iceberg_optimization_properties` after provisioning.\n"]
    pub fn iceberg_optimization_properties(
        &self,
    ) -> terrars::ListRef<GlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_optimization_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalog_CreateTableDefaultPermissions_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_principal_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalog_CreateTableDefaultPermissions_Principal {
    #[doc = "Set the field `data_lake_principal_identifier`.\n"]
    pub fn set_data_lake_principal_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_lake_principal_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CreateTableDefaultPermissions_Principal {
    type O = terrars::BlockAssignable<GlueCatalog_CreateTableDefaultPermissions_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CreateTableDefaultPermissions_Principal {}

impl BuildGlueCatalog_CreateTableDefaultPermissions_Principal {
    pub fn build(self) -> GlueCatalog_CreateTableDefaultPermissions_Principal {
        GlueCatalog_CreateTableDefaultPermissions_Principal {
            data_lake_principal_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
        GlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_lake_principal_identifier` after provisioning.\n"]
    pub fn data_lake_principal_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_principal_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalog_CreateDatabaseDefaultPermissionsDynamic {
    principal: std::option::Option<terrars::DynamicBlock<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
}

#[derive(Serialize)]
pub struct GlueCatalog_CreateDatabaseDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
    dynamic: GlueCatalog_CreateDatabaseDefaultPermissionsDynamic,
}

impl GlueCatalog_CreateDatabaseDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CreateDatabaseDefaultPermissions {
    type O = terrars::BlockAssignable<GlueCatalog_CreateDatabaseDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CreateDatabaseDefaultPermissions {}

impl BuildGlueCatalog_CreateDatabaseDefaultPermissions {
    pub fn build(self) -> GlueCatalog_CreateDatabaseDefaultPermissions {
        GlueCatalog_CreateDatabaseDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CreateDatabaseDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CreateDatabaseDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalog_CreateDatabaseDefaultPermissionsRef {
        GlueCatalog_CreateDatabaseDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CreateDatabaseDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<GlueCatalog_CreateTableDefaultPermissions_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalog_CreateTableDefaultPermissionsDynamic {
    principal: std::option::Option<terrars::DynamicBlock<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
}

#[derive(Serialize)]
pub struct GlueCatalog_CreateTableDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
    dynamic: GlueCatalog_CreateTableDefaultPermissionsDynamic,
}

impl GlueCatalog_CreateTableDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalog_CreateTableDefaultPermissions_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_CreateTableDefaultPermissions {
    type O = terrars::BlockAssignable<GlueCatalog_CreateTableDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_CreateTableDefaultPermissions {}

impl BuildGlueCatalog_CreateTableDefaultPermissions {
    pub fn build(self) -> GlueCatalog_CreateTableDefaultPermissions {
        GlueCatalog_CreateTableDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_CreateTableDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_CreateTableDefaultPermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalog_CreateTableDefaultPermissionsRef {
        GlueCatalog_CreateTableDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_CreateTableDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<GlueCatalog_CreateTableDefaultPermissions_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalog_FederatedCatalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalog_FederatedCatalog {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalog_FederatedCatalog {
    type O = terrars::BlockAssignable<GlueCatalog_FederatedCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_FederatedCatalog {}

impl BuildGlueCatalog_FederatedCatalog {
    pub fn build(self) -> GlueCatalog_FederatedCatalog {
        GlueCatalog_FederatedCatalog {
            connection_name: core::default::Default::default(),
            connection_type: core::default::Default::default(),
            identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_FederatedCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_FederatedCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalog_FederatedCatalogRef {
        GlueCatalog_FederatedCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_FederatedCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalog_TargetRedshiftCatalog {
    catalog_arn: terrars::PrimField<std::string::String>,
}

impl GlueCatalog_TargetRedshiftCatalog { }

impl terrars::ToListMappable for GlueCatalog_TargetRedshiftCatalog {
    type O = terrars::BlockAssignable<GlueCatalog_TargetRedshiftCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_TargetRedshiftCatalog {
    #[doc = ""]
    pub catalog_arn: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalog_TargetRedshiftCatalog {
    pub fn build(self) -> GlueCatalog_TargetRedshiftCatalog {
        GlueCatalog_TargetRedshiftCatalog { catalog_arn: self.catalog_arn }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_TargetRedshiftCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_TargetRedshiftCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalog_TargetRedshiftCatalogRef {
        GlueCatalog_TargetRedshiftCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_TargetRedshiftCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_arn` after provisioning.\n"]
    pub fn catalog_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalog_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalog_Timeouts {
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

impl terrars::ToListMappable for GlueCatalog_Timeouts {
    type O = terrars::BlockAssignable<GlueCatalog_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalog_Timeouts {}

impl BuildGlueCatalog_Timeouts {
    pub fn build(self) -> GlueCatalog_Timeouts {
        GlueCatalog_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalog_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalog_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalog_TimeoutsRef {
        GlueCatalog_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalog_TimeoutsRef {
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
