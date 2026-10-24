use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogDatabaseData {
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
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_table_default_permission: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_database: std::option::Option<std::vec::Vec<GlueCatalogDatabaseTargetDatabaseEl>>,
    dynamic: GlueCatalogDatabaseDynamic,
}

struct GlueCatalogDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCatalogDatabaseData>,
}

#[derive(Clone)]
pub struct GlueCatalogDatabase(std::rc::Rc<GlueCatalogDatabase_>);

impl GlueCatalogDatabase {
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

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_uri`.\n"]
    pub fn set_location_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location_uri = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `create_table_default_permission`.\n"]
    pub fn set_create_table_default_permission(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_table_default_permission = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.create_table_default_permission = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_database`.\n"]
    pub fn set_target_database(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogDatabaseTargetDatabaseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_table_default_permission` after provisioning.\n"]
    pub fn create_table_default_permission(
        &self,
    ) -> terrars::ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permission", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_database` after provisioning.\n"]
    pub fn target_database(&self) -> terrars::ListRef<GlueCatalogDatabaseTargetDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_database", self.extract_ref()))
    }
}

impl terrars::Referable for GlueCatalogDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueCatalogDatabase { }

impl terrars::ToListMappable for GlueCatalogDatabase {
    type O = terrars::ListRef<GlueCatalogDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueCatalogDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_catalog_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalogDatabase {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueCatalogDatabase {
        let out = GlueCatalogDatabase(std::rc::Rc::new(GlueCatalogDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueCatalogDatabaseData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location_uri: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                create_table_default_permission: core::default::Default::default(),
                target_database: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueCatalogDatabaseRef {
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

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_table_default_permission` after provisioning.\n"]
    pub fn create_table_default_permission(
        &self,
    ) -> terrars::ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permission", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_database` after provisioning.\n"]
    pub fn target_database(&self) -> terrars::ListRef<GlueCatalogDatabaseTargetDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_database", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_principal_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    #[doc = "Set the field `data_lake_principal_identifier`.\n"]
    pub fn set_data_lake_principal_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_lake_principal_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    type O = terrars::BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {}

impl BuildGlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    pub fn build(self) -> GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
        GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
            data_lake_principal_identifier: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
        GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_lake_principal_identifier` after provisioning.\n"]
    pub fn data_lake_principal_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_principal_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogDatabaseCreateTableDefaultPermissionElDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
    dynamic: GlueCatalogDatabaseCreateTableDefaultPermissionElDynamic,
}

impl GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
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

impl terrars::ToListMappable for GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    type O = terrars::BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogDatabaseCreateTableDefaultPermissionEl {}

impl BuildGlueCatalogDatabaseCreateTableDefaultPermissionEl {
    pub fn build(self) -> GlueCatalogDatabaseCreateTableDefaultPermissionEl {
        GlueCatalogDatabaseCreateTableDefaultPermissionEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
        GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogDatabaseTargetDatabaseEl {
    catalog_id: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
}

impl GlueCatalogDatabaseTargetDatabaseEl { }

impl terrars::ToListMappable for GlueCatalogDatabaseTargetDatabaseEl {
    type O = terrars::BlockAssignable<GlueCatalogDatabaseTargetDatabaseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogDatabaseTargetDatabaseEl {
    #[doc = ""]
    pub catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogDatabaseTargetDatabaseEl {
    pub fn build(self) -> GlueCatalogDatabaseTargetDatabaseEl {
        GlueCatalogDatabaseTargetDatabaseEl {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
        }
    }
}

pub struct GlueCatalogDatabaseTargetDatabaseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogDatabaseTargetDatabaseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogDatabaseTargetDatabaseElRef {
        GlueCatalogDatabaseTargetDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogDatabaseTargetDatabaseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogDatabaseDynamic {
    create_table_default_permission: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    target_database: std::option::Option<terrars::DynamicBlock<GlueCatalogDatabaseTargetDatabaseEl>>,
}
