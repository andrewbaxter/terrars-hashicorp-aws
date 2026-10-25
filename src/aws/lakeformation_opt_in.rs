#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationOptInData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<LakeformationOptIn_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<LakeformationOptIn_Principal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_data: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData>>,
    dynamic: LakeformationOptInDynamic,
}

struct LakeformationOptIn_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LakeformationOptInData_>,
}

#[derive(Clone)]
pub struct LakeformationOptIn(std::rc::Rc<LakeformationOptIn_>);

impl LakeformationOptIn {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_data`.\n"]
    pub fn set_resource_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_by` after provisioning.\n"]
    pub fn last_updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<LakeformationOptIn_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<LakeformationOptIn_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_data` after provisioning.\n"]
    pub fn resource_data(&self) -> terrars::ListRef<LakeformationOptIn_ResourceDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_data", self.extract_ref()))
    }
}

impl terrars::Referable for LakeformationOptIn {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LakeformationOptIn { }

impl terrars::ToListMappable for LakeformationOptIn {
    type O = terrars::ListRef<LakeformationOptInRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LakeformationOptIn_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lakeformation_opt_in".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationOptIn {
    pub tf_id: std::string::String,
}

impl BuildLakeformationOptIn {
    pub fn build(self, stack: &mut terrars::Stack) -> LakeformationOptIn {
        let out = LakeformationOptIn(std::rc::Rc::new(LakeformationOptIn_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LakeformationOptInData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                condition: core::default::Default::default(),
                principal: core::default::Default::default(),
                resource_data: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationOptInRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptInRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LakeformationOptInRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_by` after provisioning.\n"]
    pub fn last_updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<LakeformationOptIn_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<LakeformationOptIn_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_data` after provisioning.\n"]
    pub fn resource_data(&self) -> terrars::ListRef<LakeformationOptIn_ResourceDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_data", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LakeformationOptInDynamic {
    condition: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_Condition>>,
    principal: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_Principal>>,
    resource_data: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData>>,
}

#[derive(Serialize)]
pub struct LakeformationOptIn_Condition {}

impl LakeformationOptIn_Condition { }

impl terrars::ToListMappable for LakeformationOptIn_Condition {
    type O = terrars::BlockAssignable<LakeformationOptIn_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_Condition {}

impl BuildLakeformationOptIn_Condition {
    pub fn build(self) -> LakeformationOptIn_Condition {
        LakeformationOptIn_Condition {}
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ConditionRef {
        LakeformationOptIn_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_Principal {
    data_lake_principal_identifier: terrars::PrimField<std::string::String>,
}

impl LakeformationOptIn_Principal { }

impl terrars::ToListMappable for LakeformationOptIn_Principal {
    type O = terrars::BlockAssignable<LakeformationOptIn_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_Principal {
    #[doc = ""]
    pub data_lake_principal_identifier: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_Principal {
    pub fn build(self) -> LakeformationOptIn_Principal {
        LakeformationOptIn_Principal { data_lake_principal_identifier: self.data_lake_principal_identifier }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_PrincipalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_PrincipalRef {
        LakeformationOptIn_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_lake_principal_identifier` after provisioning.\n"]
    pub fn data_lake_principal_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_principal_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_Catalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationOptIn_ResourceData_Catalog {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_Catalog {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_Catalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_Catalog {}

impl BuildLakeformationOptIn_ResourceData_Catalog {
    pub fn build(self) -> LakeformationOptIn_ResourceData_Catalog {
        LakeformationOptIn_ResourceData_Catalog { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_CatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_CatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ResourceData_CatalogRef {
        LakeformationOptIn_ResourceData_CatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_CatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_DataCellsFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationOptIn_ResourceData_DataCellsFilter {
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_catalog_id`.\n"]
    pub fn set_table_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_DataCellsFilter {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_DataCellsFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_DataCellsFilter {}

impl BuildLakeformationOptIn_ResourceData_DataCellsFilter {
    pub fn build(self) -> LakeformationOptIn_ResourceData_DataCellsFilter {
        LakeformationOptIn_ResourceData_DataCellsFilter {
            database_name: core::default::Default::default(),
            name: core::default::Default::default(),
            table_catalog_id: core::default::Default::default(),
            table_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_DataCellsFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_DataCellsFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_DataCellsFilterRef {
        LakeformationOptIn_ResourceData_DataCellsFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_DataCellsFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `table_catalog_id` after provisioning.\n"]
    pub fn table_catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_DataLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_arn: terrars::PrimField<std::string::String>,
}

impl LakeformationOptIn_ResourceData_DataLocation {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_DataLocation {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_DataLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_DataLocation {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_DataLocation {
    pub fn build(self) -> LakeformationOptIn_ResourceData_DataLocation {
        LakeformationOptIn_ResourceData_DataLocation {
            catalog_id: core::default::Default::default(),
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_DataLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_DataLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_DataLocationRef {
        LakeformationOptIn_ResourceData_DataLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_DataLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_Database {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl LakeformationOptIn_ResourceData_Database {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_Database {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_Database {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_Database {
    pub fn build(self) -> LakeformationOptIn_ResourceData_Database {
        LakeformationOptIn_ResourceData_Database {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ResourceData_DatabaseRef {
        LakeformationOptIn_ResourceData_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_DatabaseRef {
    #[allow(dead_code)]
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
pub struct LakeformationOptIn_ResourceData_LfTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LakeformationOptIn_ResourceData_LfTag {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_LfTag {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_LfTag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLakeformationOptIn_ResourceData_LfTag {
    pub fn build(self) -> LakeformationOptIn_ResourceData_LfTag {
        LakeformationOptIn_ResourceData_LfTag {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_LfTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_LfTagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ResourceData_LfTagRef {
        LakeformationOptIn_ResourceData_LfTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_LfTagRef {
    #[allow(dead_code)]
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
pub struct LakeformationOptIn_ResourceData_LfTagExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl LakeformationOptIn_ResourceData_LfTagExpression {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_LfTagExpression {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTagExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_LfTagExpression {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_LfTagExpression {
    pub fn build(self) -> LakeformationOptIn_ResourceData_LfTagExpression {
        LakeformationOptIn_ResourceData_LfTagExpression {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_LfTagExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_LfTagExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_LfTagExpressionRef {
        LakeformationOptIn_ResourceData_LfTagExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_LfTagExpressionRef {
    #[allow(dead_code)]
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
pub struct LakeformationOptIn_ResourceData_LfTagPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression_name: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
}

impl LakeformationOptIn_ResourceData_LfTagPolicy {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression_name`.\n"]
    pub fn set_expression_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_LfTagPolicy {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTagPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_LfTagPolicy {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_LfTagPolicy {
    pub fn build(self) -> LakeformationOptIn_ResourceData_LfTagPolicy {
        LakeformationOptIn_ResourceData_LfTagPolicy {
            catalog_id: core::default::Default::default(),
            expression: core::default::Default::default(),
            expression_name: core::default::Default::default(),
            resource_type: self.resource_type,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_LfTagPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_LfTagPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_LfTagPolicyRef {
        LakeformationOptIn_ResourceData_LfTagPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_LfTagPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `expression_name` after provisioning.\n"]
    pub fn expression_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_Table {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl LakeformationOptIn_ResourceData_Table {
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

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_Table {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_Table>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_Table {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_Table {
    pub fn build(self) -> LakeformationOptIn_ResourceData_Table {
        LakeformationOptIn_ResourceData_Table {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_TableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ResourceData_TableRef {
        LakeformationOptIn_ResourceData_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_TableRef {
    #[allow(dead_code)]
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
pub struct LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_column_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {}

impl BuildLakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
    pub fn build(self) -> LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
        LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard {
            excluded_column_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef {
        LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_column_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationOptIn_ResourceData_TableWithColumnsDynamic {
    column_wildcard: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard>>,
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData_TableWithColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_wildcard: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard>>,
    dynamic: LakeformationOptIn_ResourceData_TableWithColumnsDynamic,
}

impl LakeformationOptIn_ResourceData_TableWithColumns {
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

    #[doc = "Set the field `column_wildcard`.\n"]
    pub fn set_column_wildcard(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcard>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_wildcard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_wildcard = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData_TableWithColumns {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData_TableWithColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData_TableWithColumns {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationOptIn_ResourceData_TableWithColumns {
    pub fn build(self) -> LakeformationOptIn_ResourceData_TableWithColumns {
        LakeformationOptIn_ResourceData_TableWithColumns {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            name: self.name,
            column_wildcard: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceData_TableWithColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceData_TableWithColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationOptIn_ResourceData_TableWithColumnsRef {
        LakeformationOptIn_ResourceData_TableWithColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceData_TableWithColumnsRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `column_wildcard` after provisioning.\n"]
    pub fn column_wildcard(
        &self,
    ) -> terrars::ListRef<LakeformationOptIn_ResourceData_TableWithColumns_ColumnWildcardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_wildcard", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationOptIn_ResourceDataDynamic {
    catalog: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_Catalog>>,
    data_cells_filter: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_DataCellsFilter>>,
    data_location: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_DataLocation>>,
    database: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_Database>>,
    lf_tag: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_LfTag>>,
    lf_tag_expression: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_LfTagExpression>>,
    lf_tag_policy: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_LfTagPolicy>>,
    table: std::option::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_Table>>,
    table_with_columns: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationOptIn_ResourceData_TableWithColumns>>,
}

#[derive(Serialize)]
pub struct LakeformationOptIn_ResourceData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_Catalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cells_filter: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_DataCellsFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_location: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_DataLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_LfTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag_expression: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_LfTagExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag_policy: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_LfTagPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_Table>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_with_columns: std::option::Option<std::vec::Vec<LakeformationOptIn_ResourceData_TableWithColumns>>,
    dynamic: LakeformationOptIn_ResourceDataDynamic,
}

impl LakeformationOptIn_ResourceData {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_Catalog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_cells_filter`.\n"]
    pub fn set_data_cells_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_DataCellsFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_cells_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cells_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_location`.\n"]
    pub fn set_data_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_DataLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_Database>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag`.\n"]
    pub fn set_lf_tag(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lf_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag_expression`.\n"]
    pub fn set_lf_tag_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTagExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lf_tag_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag_policy`.\n"]
    pub fn set_lf_tag_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_LfTagPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lf_tag_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_Table>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_with_columns`.\n"]
    pub fn set_table_with_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationOptIn_ResourceData_TableWithColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_with_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_with_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LakeformationOptIn_ResourceData {
    type O = terrars::BlockAssignable<LakeformationOptIn_ResourceData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationOptIn_ResourceData {}

impl BuildLakeformationOptIn_ResourceData {
    pub fn build(self) -> LakeformationOptIn_ResourceData {
        LakeformationOptIn_ResourceData {
            catalog: core::default::Default::default(),
            data_cells_filter: core::default::Default::default(),
            data_location: core::default::Default::default(),
            database: core::default::Default::default(),
            lf_tag: core::default::Default::default(),
            lf_tag_expression: core::default::Default::default(),
            lf_tag_policy: core::default::Default::default(),
            table: core::default::Default::default(),
            table_with_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationOptIn_ResourceDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationOptIn_ResourceDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationOptIn_ResourceDataRef {
        LakeformationOptIn_ResourceDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationOptIn_ResourceDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_CatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_DataCellsFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cells_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_DataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_LfTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `lf_tag_expression` after provisioning.\n"]
    pub fn lf_tag_expression(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_LfTagExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_LfTagPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<LakeformationOptIn_ResourceData_TableWithColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.base))
    }
}
