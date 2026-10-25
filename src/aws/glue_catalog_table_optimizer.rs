#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogTableOptimizerData_ {
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
    catalog_id: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration>>,
    dynamic: GlueCatalogTableOptimizerDynamic,
}

struct GlueCatalogTableOptimizer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCatalogTableOptimizerData_>,
}

#[derive(Clone)]
pub struct GlueCatalogTableOptimizer(std::rc::Rc<GlueCatalogTableOptimizer_>);

impl GlueCatalogTableOptimizer {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<GlueCatalogTableOptimizer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for GlueCatalogTableOptimizer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueCatalogTableOptimizer { }

impl terrars::ToListMappable for GlueCatalogTableOptimizer {
    type O = terrars::ListRef<GlueCatalogTableOptimizerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueCatalogTableOptimizer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_catalog_table_optimizer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalogTableOptimizer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTableOptimizer {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueCatalogTableOptimizer {
        let out = GlueCatalogTableOptimizer(std::rc::Rc::new(GlueCatalogTableOptimizer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueCatalogTableOptimizerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: self.catalog_id,
                database_name: self.database_name,
                region: core::default::Default::default(),
                table_name: self.table_name,
                type_: self.type_,
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogTableOptimizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueCatalogTableOptimizerRef {
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

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<GlueCatalogTableOptimizer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizerDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_file_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_input_files: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
    #[doc = "Set the field `delete_file_threshold`.\n"]
    pub fn set_delete_file_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.delete_file_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_input_files`.\n"]
    pub fn set_min_input_files(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_input_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
    type O =
        terrars
        ::BlockAssignable<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
        GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration {
            delete_file_threshold: core::default::Default::default(),
            min_input_files: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_file_threshold` after provisioning.\n"]
    pub fn delete_file_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_file_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `min_input_files` after provisioning.\n"]
    pub fn min_input_files(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_input_files", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizer_Configuration_CompactionConfigurationDynamic {
    iceberg_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_configuration: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration>>,
    dynamic: GlueCatalogTableOptimizer_Configuration_CompactionConfigurationDynamic,
}

impl GlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
    type O = terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_CompactionConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
        GlueCatalogTableOptimizer_Configuration_CompactionConfiguration {
            iceberg_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(
        &self,
    ) -> terrars::ListRef<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration_IcebergConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orphan_file_retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_rate_in_hours: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orphan_file_retention_period_in_days`.\n"]
    pub fn set_orphan_file_retention_period_in_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.orphan_file_retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_rate_in_hours`.\n"]
    pub fn set_run_rate_in_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_rate_in_hours = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
    pub fn build(
        self,
    ) -> GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
        GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration {
            location: core::default::Default::default(),
            orphan_file_retention_period_in_days: core::default::Default::default(),
            run_rate_in_hours: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_retention_period_in_days` after provisioning.\n"]
    pub fn orphan_file_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.orphan_file_retention_period_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `run_rate_in_hours` after provisioning.\n"]
    pub fn run_rate_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_rate_in_hours", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationDynamic {
    iceberg_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_configuration: std
    ::option
    ::Option<
        std::vec::Vec<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration>,
    >,
    dynamic: GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationDynamic,
}

impl GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
    type O = terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
        GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration {
            iceberg_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(
        &self,
    ) -> terrars
    ::ListRef<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration_IcebergConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clean_expired_files: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_snapshots_to_retain: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_rate_in_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
    #[doc = "Set the field `clean_expired_files`.\n"]
    pub fn set_clean_expired_files(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.clean_expired_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_snapshots_to_retain`.\n"]
    pub fn set_number_of_snapshots_to_retain(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_snapshots_to_retain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_rate_in_hours`.\n"]
    pub fn set_run_rate_in_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_rate_in_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_retention_period_in_days`.\n"]
    pub fn set_snapshot_retention_period_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.snapshot_retention_period_in_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
    type O =
        terrars
        ::BlockAssignable<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
        GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration {
            clean_expired_files: core::default::Default::default(),
            number_of_snapshots_to_retain: core::default::Default::default(),
            run_rate_in_hours: core::default::Default::default(),
            snapshot_retention_period_in_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clean_expired_files` after provisioning.\n"]
    pub fn clean_expired_files(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clean_expired_files", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_snapshots_to_retain` after provisioning.\n"]
    pub fn number_of_snapshots_to_retain(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_snapshots_to_retain", self.base))
    }

    #[doc = "Get a reference to the value of field `run_rate_in_hours` after provisioning.\n"]
    pub fn run_rate_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_rate_in_hours", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_period_in_days` after provisioning.\n"]
    pub fn snapshot_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_retention_period_in_days", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizer_Configuration_RetentionConfigurationDynamic {
    iceberg_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_configuration: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration>>,
    dynamic: GlueCatalogTableOptimizer_Configuration_RetentionConfigurationDynamic,
}

impl GlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
    type O = terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration_RetentionConfiguration {}

impl BuildGlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
        GlueCatalogTableOptimizer_Configuration_RetentionConfiguration {
            iceberg_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef {
        GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(
        &self,
    ) -> terrars::ListRef<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration_IcebergConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizer_ConfigurationDynamic {
    compaction_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration>>,
    orphan_file_deletion_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration>>,
    retention_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizer_Configuration {
    enabled: terrars::PrimField<bool>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compaction_configuration: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orphan_file_deletion_configuration: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_configuration: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration>>,
    dynamic: GlueCatalogTableOptimizer_ConfigurationDynamic,
}

impl GlueCatalogTableOptimizer_Configuration {
    #[doc = "Set the field `compaction_configuration`.\n"]
    pub fn set_compaction_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration_CompactionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compaction_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compaction_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `orphan_file_deletion_configuration`.\n"]
    pub fn set_orphan_file_deletion_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.orphan_file_deletion_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.orphan_file_deletion_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_configuration`.\n"]
    pub fn set_retention_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration_RetentionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retention_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTableOptimizer_Configuration {
    type O = terrars::BlockAssignable<GlueCatalogTableOptimizer_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizer_Configuration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTableOptimizer_Configuration {
    pub fn build(self) -> GlueCatalogTableOptimizer_Configuration {
        GlueCatalogTableOptimizer_Configuration {
            enabled: self.enabled,
            role_arn: self.role_arn,
            compaction_configuration: core::default::Default::default(),
            orphan_file_deletion_configuration: core::default::Default::default(),
            retention_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTableOptimizer_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableOptimizer_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTableOptimizer_ConfigurationRef {
        GlueCatalogTableOptimizer_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizer_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `compaction_configuration` after provisioning.\n"]
    pub fn compaction_configuration(
        &self,
    ) -> terrars::ListRef<GlueCatalogTableOptimizer_Configuration_CompactionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compaction_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_deletion_configuration` after provisioning.\n"]
    pub fn orphan_file_deletion_configuration(
        &self,
    ) -> terrars::ListRef<GlueCatalogTableOptimizer_Configuration_OrphanFileDeletionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orphan_file_deletion_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_configuration` after provisioning.\n"]
    pub fn retention_configuration(
        &self,
    ) -> terrars::ListRef<GlueCatalogTableOptimizer_Configuration_RetentionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_configuration", self.base))
    }
}
