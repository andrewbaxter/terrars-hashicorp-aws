use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLakeformationPermissionsData {
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
    catalog_resource: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_location: std::option::Option<std::vec::Vec<DataLakeformationPermissionsDataLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<DataLakeformationPermissionsDatabaseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag: std::option::Option<std::vec::Vec<DataLakeformationPermissionsLfTagEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag_policy: std::option::Option<std::vec::Vec<DataLakeformationPermissionsLfTagPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<DataLakeformationPermissionsTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_with_columns: std::option::Option<std::vec::Vec<DataLakeformationPermissionsTableWithColumnsEl>>,
    dynamic: DataLakeformationPermissionsDynamic,
}

struct DataLakeformationPermissions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLakeformationPermissionsData>,
}

#[derive(Clone)]
pub struct DataLakeformationPermissions(std::rc::Rc<DataLakeformationPermissions_>);

impl DataLakeformationPermissions {
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

    #[doc = "Set the field `catalog_resource`.\n"]
    pub fn set_catalog_resource(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().catalog_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_location`.\n"]
    pub fn set_data_location(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsDataLocationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsDatabaseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag`.\n"]
    pub fn set_lf_tag(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsLfTagEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag_policy`.\n"]
    pub fn set_lf_tag_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsLfTagPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsTableEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_with_columns`.\n"]
    pub fn set_table_with_columns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsTableWithColumnsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_with_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_with_columns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<DataLakeformationPermissionsDataLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataLakeformationPermissionsDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<DataLakeformationPermissionsLfTagElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<DataLakeformationPermissionsLfTagPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<DataLakeformationPermissionsTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<DataLakeformationPermissionsTableWithColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

impl terrars::Referable for DataLakeformationPermissions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLakeformationPermissions { }

impl terrars::ToListMappable for DataLakeformationPermissions {
    type O = terrars::ListRef<DataLakeformationPermissionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLakeformationPermissions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lakeformation_permissions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLakeformationPermissions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLakeformationPermissions {
        let out = DataLakeformationPermissions(std::rc::Rc::new(DataLakeformationPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLakeformationPermissionsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: core::default::Default::default(),
                catalog_resource: core::default::Default::default(),
                id: core::default::Default::default(),
                principal: self.principal,
                data_location: core::default::Default::default(),
                database: core::default::Default::default(),
                lf_tag: core::default::Default::default(),
                lf_tag_policy: core::default::Default::default(),
                table: core::default::Default::default(),
                table_with_columns: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLakeformationPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLakeformationPermissionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<DataLakeformationPermissionsDataLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataLakeformationPermissionsDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<DataLakeformationPermissionsLfTagElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<DataLakeformationPermissionsLfTagPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<DataLakeformationPermissionsTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<DataLakeformationPermissionsTableWithColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsDataLocationEl {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissionsDataLocationEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsDataLocationEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsDataLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsDataLocationEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissionsDataLocationEl {
    pub fn build(self) -> DataLakeformationPermissionsDataLocationEl {
        DataLakeformationPermissionsDataLocationEl {
            arn: self.arn,
            catalog_id: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationPermissionsDataLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsDataLocationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissionsDataLocationElRef {
        DataLakeformationPermissionsDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsDataLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsDatabaseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl DataLakeformationPermissionsDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsDatabaseEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsDatabaseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsDatabaseEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissionsDatabaseEl {
    pub fn build(self) -> DataLakeformationPermissionsDatabaseEl {
        DataLakeformationPermissionsDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct DataLakeformationPermissionsDatabaseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsDatabaseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissionsDatabaseElRef {
        DataLakeformationPermissionsDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsDatabaseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissionsLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsLfTagEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsLfTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsLfTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLakeformationPermissionsLfTagEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagEl {
        DataLakeformationPermissionsLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}

pub struct DataLakeformationPermissionsLfTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsLfTagElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissionsLfTagElRef {
        DataLakeformationPermissionsLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsLfTagElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagPolicyElExpressionEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissionsLfTagPolicyElExpressionEl { }

impl terrars::ToListMappable for DataLakeformationPermissionsLfTagPolicyElExpressionEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsLfTagPolicyElExpressionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsLfTagPolicyElExpressionEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLakeformationPermissionsLfTagPolicyElExpressionEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagPolicyElExpressionEl {
        DataLakeformationPermissionsLfTagPolicyElExpressionEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
        DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLakeformationPermissionsLfTagPolicyElDynamic {
    expression: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<std::vec::Vec<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
    dynamic: DataLakeformationPermissionsLfTagPolicyElDynamic,
}

impl DataLakeformationPermissionsLfTagPolicyEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsLfTagPolicyEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsLfTagPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsLfTagPolicyEl {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissionsLfTagPolicyEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagPolicyEl {
        DataLakeformationPermissionsLfTagPolicyEl {
            catalog_id: core::default::Default::default(),
            resource_type: self.resource_type,
            expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationPermissionsLfTagPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsLfTagPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissionsLfTagPolicyElRef {
        DataLakeformationPermissionsLfTagPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsLfTagPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::ListRef<DataLakeformationPermissionsLfTagPolicyElExpressionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLakeformationPermissionsTableEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsTableEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsTableEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissionsTableEl {
    pub fn build(self) -> DataLakeformationPermissionsTableEl {
        DataLakeformationPermissionsTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationPermissionsTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsTableElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissionsTableElRef {
        DataLakeformationPermissionsTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsTableElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissionsTableWithColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLakeformationPermissionsTableWithColumnsEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissionsTableWithColumnsEl {
    type O = terrars::BlockAssignable<DataLakeformationPermissionsTableWithColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissionsTableWithColumnsEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissionsTableWithColumnsEl {
    pub fn build(self) -> DataLakeformationPermissionsTableWithColumnsEl {
        DataLakeformationPermissionsTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            excluded_column_names: core::default::Default::default(),
            name: self.name,
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationPermissionsTableWithColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsTableWithColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationPermissionsTableWithColumnsElRef {
        DataLakeformationPermissionsTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissionsTableWithColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLakeformationPermissionsDynamic {
    data_location: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsDataLocationEl>>,
    database: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsDatabaseEl>>,
    lf_tag: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsLfTagEl>>,
    lf_tag_policy: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsLfTagPolicyEl>>,
    table: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsTableEl>>,
    table_with_columns: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissionsTableWithColumnsEl>>,
}
