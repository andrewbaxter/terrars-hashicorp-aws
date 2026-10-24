use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationResourceLfTagsData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<LakeformationResourceLfTagsDatabaseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag: std::option::Option<std::vec::Vec<LakeformationResourceLfTagsLfTagEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<LakeformationResourceLfTagsTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_with_columns: std::option::Option<std::vec::Vec<LakeformationResourceLfTagsTableWithColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LakeformationResourceLfTagsTimeoutsEl>,
    dynamic: LakeformationResourceLfTagsDynamic,
}

struct LakeformationResourceLfTags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LakeformationResourceLfTagsData>,
}

#[derive(Clone)]
pub struct LakeformationResourceLfTags(std::rc::Rc<LakeformationResourceLfTags_>);

impl LakeformationResourceLfTags {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationResourceLfTagsDatabaseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationResourceLfTagsLfTagEl>>,
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

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationResourceLfTagsTableEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationResourceLfTagsTableWithColumnsEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LakeformationResourceLfTagsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<LakeformationResourceLfTagsDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<LakeformationResourceLfTagsTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<LakeformationResourceLfTagsTableWithColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationResourceLfTagsTimeoutsElRef {
        LakeformationResourceLfTagsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for LakeformationResourceLfTags {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LakeformationResourceLfTags { }

impl terrars::ToListMappable for LakeformationResourceLfTags {
    type O = terrars::ListRef<LakeformationResourceLfTagsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LakeformationResourceLfTags_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lakeformation_resource_lf_tags".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationResourceLfTags {
    pub tf_id: std::string::String,
}

impl BuildLakeformationResourceLfTags {
    pub fn build(self, stack: &mut terrars::Stack) -> LakeformationResourceLfTags {
        let out = LakeformationResourceLfTags(std::rc::Rc::new(LakeformationResourceLfTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LakeformationResourceLfTagsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                id: core::default::Default::default(),
                database: core::default::Default::default(),
                lf_tag: core::default::Default::default(),
                table: core::default::Default::default(),
                table_with_columns: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationResourceLfTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LakeformationResourceLfTagsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<LakeformationResourceLfTagsDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<LakeformationResourceLfTagsTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<LakeformationResourceLfTagsTableWithColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationResourceLfTagsTimeoutsElRef {
        LakeformationResourceLfTagsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LakeformationResourceLfTagsDatabaseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl LakeformationResourceLfTagsDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationResourceLfTagsDatabaseEl {
    type O = terrars::BlockAssignable<LakeformationResourceLfTagsDatabaseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationResourceLfTagsDatabaseEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationResourceLfTagsDatabaseEl {
    pub fn build(self) -> LakeformationResourceLfTagsDatabaseEl {
        LakeformationResourceLfTagsDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct LakeformationResourceLfTagsDatabaseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsDatabaseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationResourceLfTagsDatabaseElRef {
        LakeformationResourceLfTagsDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationResourceLfTagsDatabaseElRef {
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
pub struct LakeformationResourceLfTagsLfTagEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl LakeformationResourceLfTagsLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationResourceLfTagsLfTagEl {
    type O = terrars::BlockAssignable<LakeformationResourceLfTagsLfTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationResourceLfTagsLfTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationResourceLfTagsLfTagEl {
    pub fn build(self) -> LakeformationResourceLfTagsLfTagEl {
        LakeformationResourceLfTagsLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            value: self.value,
        }
    }
}

pub struct LakeformationResourceLfTagsLfTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsLfTagElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationResourceLfTagsLfTagElRef {
        LakeformationResourceLfTagsLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationResourceLfTagsLfTagElRef {
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

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationResourceLfTagsTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl LakeformationResourceLfTagsTableEl {
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

impl terrars::ToListMappable for LakeformationResourceLfTagsTableEl {
    type O = terrars::BlockAssignable<LakeformationResourceLfTagsTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationResourceLfTagsTableEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationResourceLfTagsTableEl {
    pub fn build(self) -> LakeformationResourceLfTagsTableEl {
        LakeformationResourceLfTagsTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct LakeformationResourceLfTagsTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsTableElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationResourceLfTagsTableElRef {
        LakeformationResourceLfTagsTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationResourceLfTagsTableElRef {
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
pub struct LakeformationResourceLfTagsTableWithColumnsEl {
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

impl LakeformationResourceLfTagsTableWithColumnsEl {
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

impl terrars::ToListMappable for LakeformationResourceLfTagsTableWithColumnsEl {
    type O = terrars::BlockAssignable<LakeformationResourceLfTagsTableWithColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationResourceLfTagsTableWithColumnsEl {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationResourceLfTagsTableWithColumnsEl {
    pub fn build(self) -> LakeformationResourceLfTagsTableWithColumnsEl {
        LakeformationResourceLfTagsTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            excluded_column_names: core::default::Default::default(),
            name: self.name,
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct LakeformationResourceLfTagsTableWithColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsTableWithColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationResourceLfTagsTableWithColumnsElRef {
        LakeformationResourceLfTagsTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationResourceLfTagsTableWithColumnsElRef {
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

#[derive(Serialize)]
pub struct LakeformationResourceLfTagsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationResourceLfTagsTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationResourceLfTagsTimeoutsEl {
    type O = terrars::BlockAssignable<LakeformationResourceLfTagsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationResourceLfTagsTimeoutsEl {}

impl BuildLakeformationResourceLfTagsTimeoutsEl {
    pub fn build(self) -> LakeformationResourceLfTagsTimeoutsEl {
        LakeformationResourceLfTagsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct LakeformationResourceLfTagsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationResourceLfTagsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationResourceLfTagsTimeoutsElRef {
        LakeformationResourceLfTagsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationResourceLfTagsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationResourceLfTagsDynamic {
    database: std::option::Option<terrars::DynamicBlock<LakeformationResourceLfTagsDatabaseEl>>,
    lf_tag: std::option::Option<terrars::DynamicBlock<LakeformationResourceLfTagsLfTagEl>>,
    table: std::option::Option<terrars::DynamicBlock<LakeformationResourceLfTagsTableEl>>,
    table_with_columns: std::option::Option<terrars::DynamicBlock<LakeformationResourceLfTagsTableWithColumnsEl>>,
}
