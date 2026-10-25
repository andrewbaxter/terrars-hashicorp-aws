#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BcmdataexportsExportData_ {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export: std::option::Option<std::vec::Vec<BcmdataexportsExport_Export>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BcmdataexportsExport_Timeouts>,
    dynamic: BcmdataexportsExportDynamic,
}

struct BcmdataexportsExport_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BcmdataexportsExportData_>,
}

#[derive(Clone)]
pub struct BcmdataexportsExport(std::rc::Rc<BcmdataexportsExport_>);

impl BcmdataexportsExport {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export`.\n"]
    pub fn set_export(self, v: impl std::convert::Into<terrars::BlockAssignable<BcmdataexportsExport_Export>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BcmdataexportsExport_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> terrars::ListRef<BcmdataexportsExport_ExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BcmdataexportsExport_TimeoutsRef {
        BcmdataexportsExport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BcmdataexportsExport {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BcmdataexportsExport { }

impl terrars::ToListMappable for BcmdataexportsExport {
    type O = terrars::ListRef<BcmdataexportsExportRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BcmdataexportsExport_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bcmdataexports_export".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBcmdataexportsExport {
    pub tf_id: std::string::String,
}

impl BuildBcmdataexportsExport {
    pub fn build(self, stack: &mut terrars::Stack) -> BcmdataexportsExport {
        let out = BcmdataexportsExport(std::rc::Rc::new(BcmdataexportsExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BcmdataexportsExportData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                tags: core::default::Default::default(),
                export: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BcmdataexportsExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BcmdataexportsExportRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> terrars::ListRef<BcmdataexportsExport_ExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BcmdataexportsExport_TimeoutsRef {
        BcmdataexportsExport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BcmdataexportsExportDynamic {
    export: std::option::Option<terrars::DynamicBlock<BcmdataexportsExport_Export>>,
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export_DataQuery {
    query_statement: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_configurations: std
    ::option
    ::Option<terrars::RecField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
}

impl BcmdataexportsExport_Export_DataQuery {
    #[doc = "Set the field `table_configurations`.\n"]
    pub fn set_table_configurations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.table_configurations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BcmdataexportsExport_Export_DataQuery {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Export_DataQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export_DataQuery {
    #[doc = ""]
    pub query_statement: terrars::PrimField<std::string::String>,
}

impl BuildBcmdataexportsExport_Export_DataQuery {
    pub fn build(self) -> BcmdataexportsExport_Export_DataQuery {
        BcmdataexportsExport_Export_DataQuery {
            query_statement: self.query_statement,
            table_configurations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_Export_DataQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_Export_DataQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BcmdataexportsExport_Export_DataQueryRef {
        BcmdataexportsExport_Export_DataQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_Export_DataQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_statement` after provisioning.\n"]
    pub fn query_statement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `table_configurations` after provisioning.\n"]
    pub fn table_configurations(&self) -> terrars::RecRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.table_configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
    compression: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
    output_type: terrars::PrimField<std::string::String>,
    overwrite: terrars::PrimField<std::string::String>,
}

impl BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations { }

impl terrars
::ToListMappable for BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
    type O =
        terrars
        ::BlockAssignable<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
    #[doc = ""]
    pub compression: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub output_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub overwrite: terrars::PrimField<std::string::String>,
}

impl BuildBcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
    pub fn build(self) -> BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
        BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations {
            compression: self.compression,
            format: self.format,
            output_type: self.output_type,
            overwrite: self.overwrite,
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef {
        BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite` after provisioning.\n"]
    pub fn overwrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationDynamic {
    s3_output_configurations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations>,
    >,
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
    s3_bucket: terrars::PrimField<std::string::String>,
    s3_prefix: terrars::PrimField<std::string::String>,
    s3_region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_configurations: std
    ::option
    ::Option<
        std::vec::Vec<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations>,
    >,
    dynamic: BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationDynamic,
}

impl BcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
    #[doc = "Set the field `s3_output_configurations`.\n"]
    pub fn set_s3_output_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_configurations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_prefix: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_region: terrars::PrimField<std::string::String>,
}

impl BuildBcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
    pub fn build(self) -> BcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
        BcmdataexportsExport_Export_DestinationConfigurations_S3Destination {
            s3_bucket: self.s3_bucket,
            s3_prefix: self.s3_prefix,
            s3_region: self.s3_region,
            s3_output_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef {
        BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_configurations` after provisioning.\n"]
    pub fn s3_output_configurations(
        &self,
    ) -> terrars
    ::ListRef<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination_S3OutputConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_configurations", self.base))
    }
}

#[derive(Serialize, Default)]
struct BcmdataexportsExport_Export_DestinationConfigurationsDynamic {
    s3_destination: std
    ::option
    ::Option<terrars::DynamicBlock<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination>>,
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export_DestinationConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std
    ::option
    ::Option<std::vec::Vec<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination>>,
    dynamic: BcmdataexportsExport_Export_DestinationConfigurationsDynamic,
}

impl BcmdataexportsExport_Export_DestinationConfigurations {
    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BcmdataexportsExport_Export_DestinationConfigurations_S3Destination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BcmdataexportsExport_Export_DestinationConfigurations {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Export_DestinationConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export_DestinationConfigurations {}

impl BuildBcmdataexportsExport_Export_DestinationConfigurations {
    pub fn build(self) -> BcmdataexportsExport_Export_DestinationConfigurations {
        BcmdataexportsExport_Export_DestinationConfigurations {
            s3_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_Export_DestinationConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_Export_DestinationConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BcmdataexportsExport_Export_DestinationConfigurationsRef {
        BcmdataexportsExport_Export_DestinationConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_Export_DestinationConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(
        &self,
    ) -> terrars::ListRef<BcmdataexportsExport_Export_DestinationConfigurations_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export_RefreshCadence {
    frequency: terrars::PrimField<std::string::String>,
}

impl BcmdataexportsExport_Export_RefreshCadence { }

impl terrars::ToListMappable for BcmdataexportsExport_Export_RefreshCadence {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Export_RefreshCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export_RefreshCadence {
    #[doc = ""]
    pub frequency: terrars::PrimField<std::string::String>,
}

impl BuildBcmdataexportsExport_Export_RefreshCadence {
    pub fn build(self) -> BcmdataexportsExport_Export_RefreshCadence {
        BcmdataexportsExport_Export_RefreshCadence { frequency: self.frequency }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_Export_RefreshCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_Export_RefreshCadenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BcmdataexportsExport_Export_RefreshCadenceRef {
        BcmdataexportsExport_Export_RefreshCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_Export_RefreshCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}

#[derive(Serialize, Default)]
struct BcmdataexportsExport_ExportDynamic {
    data_query: std::option::Option<terrars::DynamicBlock<BcmdataexportsExport_Export_DataQuery>>,
    destination_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<BcmdataexportsExport_Export_DestinationConfigurations>>,
    refresh_cadence: std::option::Option<terrars::DynamicBlock<BcmdataexportsExport_Export_RefreshCadence>>,
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Export {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_query: std::option::Option<std::vec::Vec<BcmdataexportsExport_Export_DataQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configurations: std::option::Option<std::vec::Vec<BcmdataexportsExport_Export_DestinationConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_cadence: std::option::Option<std::vec::Vec<BcmdataexportsExport_Export_RefreshCadence>>,
    dynamic: BcmdataexportsExport_ExportDynamic,
}

impl BcmdataexportsExport_Export {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_query`.\n"]
    pub fn set_data_query(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BcmdataexportsExport_Export_DataQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_configurations`.\n"]
    pub fn set_destination_configurations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BcmdataexportsExport_Export_DestinationConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `refresh_cadence`.\n"]
    pub fn set_refresh_cadence(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BcmdataexportsExport_Export_RefreshCadence>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BcmdataexportsExport_Export {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Export>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Export {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBcmdataexportsExport_Export {
    pub fn build(self) -> BcmdataexportsExport_Export {
        BcmdataexportsExport_Export {
            description: core::default::Default::default(),
            name: self.name,
            data_query: core::default::Default::default(),
            destination_configurations: core::default::Default::default(),
            refresh_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_ExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_ExportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BcmdataexportsExport_ExportRef {
        BcmdataexportsExport_ExportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_ExportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `export_arn` after provisioning.\n"]
    pub fn export_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `data_query` after provisioning.\n"]
    pub fn data_query(&self) -> terrars::ListRef<BcmdataexportsExport_Export_DataQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_query", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_configurations` after provisioning.\n"]
    pub fn destination_configurations(
        &self,
    ) -> terrars::ListRef<BcmdataexportsExport_Export_DestinationConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configurations", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_cadence` after provisioning.\n"]
    pub fn refresh_cadence(&self) -> terrars::ListRef<BcmdataexportsExport_Export_RefreshCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_cadence", self.base))
    }
}

#[derive(Serialize)]
pub struct BcmdataexportsExport_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BcmdataexportsExport_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BcmdataexportsExport_Timeouts {
    type O = terrars::BlockAssignable<BcmdataexportsExport_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBcmdataexportsExport_Timeouts {}

impl BuildBcmdataexportsExport_Timeouts {
    pub fn build(self) -> BcmdataexportsExport_Timeouts {
        BcmdataexportsExport_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BcmdataexportsExport_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BcmdataexportsExport_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BcmdataexportsExport_TimeoutsRef {
        BcmdataexportsExport_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BcmdataexportsExport_TimeoutsRef {
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
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
