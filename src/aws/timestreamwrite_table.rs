#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TimestreamwriteTableData_ {
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
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    magnetic_store_write_properties: std
    ::option
    ::Option<std::vec::Vec<TimestreamwriteTable_MagneticStoreWriteProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_properties: std::option::Option<std::vec::Vec<TimestreamwriteTable_RetentionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<TimestreamwriteTable_Schema>>,
    dynamic: TimestreamwriteTableDynamic,
}

struct TimestreamwriteTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TimestreamwriteTableData_>,
}

#[derive(Clone)]
pub struct TimestreamwriteTable(std::rc::Rc<TimestreamwriteTable_>);

impl TimestreamwriteTable {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `magnetic_store_write_properties`.\n"]
    pub fn set_magnetic_store_write_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamwriteTable_MagneticStoreWriteProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().magnetic_store_write_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.magnetic_store_write_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_properties`.\n"]
    pub fn set_retention_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamwriteTable_RetentionProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::BlockAssignable<TimestreamwriteTable_Schema>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> terrars::ListRef<TimestreamwriteTable_MagneticStoreWritePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> terrars::ListRef<TimestreamwriteTable_RetentionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<TimestreamwriteTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }
}

impl terrars::Referable for TimestreamwriteTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TimestreamwriteTable { }

impl terrars::ToListMappable for TimestreamwriteTable {
    type O = terrars::ListRef<TimestreamwriteTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TimestreamwriteTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_timestreamwrite_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTimestreamwriteTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamwriteTable {
    pub fn build(self, stack: &mut terrars::Stack) -> TimestreamwriteTable {
        let out = TimestreamwriteTable(std::rc::Rc::new(TimestreamwriteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TimestreamwriteTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database_name: self.database_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                table_name: self.table_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                magnetic_store_write_properties: core::default::Default::default(),
                retention_properties: core::default::Default::default(),
                schema: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TimestreamwriteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TimestreamwriteTableRef {
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

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> terrars::ListRef<TimestreamwriteTable_MagneticStoreWritePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> terrars::ListRef<TimestreamwriteTable_RetentionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<TimestreamwriteTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct TimestreamwriteTableDynamic {
    magnetic_store_write_properties: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamwriteTable_MagneticStoreWriteProperties>>,
    retention_properties: std::option::Option<terrars::DynamicBlock<TimestreamwriteTable_RetentionProperties>>,
    schema: std::option::Option<terrars::DynamicBlock<TimestreamwriteTable_Schema>>,
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    type O =
        terrars
        ::BlockAssignable<
            TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {}

impl BuildTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    pub fn build(
        self,
    ) -> TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
        TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
            bucket_name: core::default::Default::default(),
            encryption_option: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
        TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationDynamic {
    s3_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
        >,
    >,
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration>,
    >,
    dynamic: TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationDynamic,
}

impl TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    type O =
        terrars
        ::BlockAssignable<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {}

impl BuildTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    pub fn build(self) -> TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
        TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
        TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars
    ::ListRef<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamwriteTable_MagneticStoreWritePropertiesDynamic {
    magnetic_store_rejected_data_location: std
    ::option
    ::Option<
        terrars::DynamicBlock<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation>,
    >,
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_MagneticStoreWriteProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_magnetic_store_writes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    magnetic_store_rejected_data_location: std
    ::option
    ::Option<std::vec::Vec<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation>>,
    dynamic: TimestreamwriteTable_MagneticStoreWritePropertiesDynamic,
}

impl TimestreamwriteTable_MagneticStoreWriteProperties {
    #[doc = "Set the field `enable_magnetic_store_writes`.\n"]
    pub fn set_enable_magnetic_store_writes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_magnetic_store_writes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `magnetic_store_rejected_data_location`.\n"]
    pub fn set_magnetic_store_rejected_data_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.magnetic_store_rejected_data_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.magnetic_store_rejected_data_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamwriteTable_MagneticStoreWriteProperties {
    type O = terrars::BlockAssignable<TimestreamwriteTable_MagneticStoreWriteProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_MagneticStoreWriteProperties {}

impl BuildTimestreamwriteTable_MagneticStoreWriteProperties {
    pub fn build(self) -> TimestreamwriteTable_MagneticStoreWriteProperties {
        TimestreamwriteTable_MagneticStoreWriteProperties {
            enable_magnetic_store_writes: core::default::Default::default(),
            magnetic_store_rejected_data_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_MagneticStoreWritePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTable_MagneticStoreWritePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamwriteTable_MagneticStoreWritePropertiesRef {
        TimestreamwriteTable_MagneticStoreWritePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_MagneticStoreWritePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_magnetic_store_writes` after provisioning.\n"]
    pub fn enable_magnetic_store_writes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_magnetic_store_writes", self.base))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_rejected_data_location` after provisioning.\n"]
    pub fn magnetic_store_rejected_data_location(
        &self,
    ) -> terrars::ListRef<TimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.magnetic_store_rejected_data_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_RetentionProperties {
    magnetic_store_retention_period_in_days: terrars::PrimField<f64>,
    memory_store_retention_period_in_hours: terrars::PrimField<f64>,
}

impl TimestreamwriteTable_RetentionProperties { }

impl terrars::ToListMappable for TimestreamwriteTable_RetentionProperties {
    type O = terrars::BlockAssignable<TimestreamwriteTable_RetentionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_RetentionProperties {
    #[doc = ""]
    pub magnetic_store_retention_period_in_days: terrars::PrimField<f64>,
    #[doc = ""]
    pub memory_store_retention_period_in_hours: terrars::PrimField<f64>,
}

impl BuildTimestreamwriteTable_RetentionProperties {
    pub fn build(self) -> TimestreamwriteTable_RetentionProperties {
        TimestreamwriteTable_RetentionProperties {
            magnetic_store_retention_period_in_days: self.magnetic_store_retention_period_in_days,
            memory_store_retention_period_in_hours: self.memory_store_retention_period_in_hours,
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_RetentionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTable_RetentionPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TimestreamwriteTable_RetentionPropertiesRef {
        TimestreamwriteTable_RetentionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_RetentionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `magnetic_store_retention_period_in_days` after provisioning.\n"]
    pub fn magnetic_store_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.magnetic_store_retention_period_in_days", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_store_retention_period_in_hours` after provisioning.\n"]
    pub fn memory_store_retention_period_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_store_retention_period_in_hours", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_Schema_CompositePartitionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcement_in_record: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl TimestreamwriteTable_Schema_CompositePartitionKey {
    #[doc = "Set the field `enforcement_in_record`.\n"]
    pub fn set_enforcement_in_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforcement_in_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TimestreamwriteTable_Schema_CompositePartitionKey {
    type O = terrars::BlockAssignable<TimestreamwriteTable_Schema_CompositePartitionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_Schema_CompositePartitionKey {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTimestreamwriteTable_Schema_CompositePartitionKey {
    pub fn build(self) -> TimestreamwriteTable_Schema_CompositePartitionKey {
        TimestreamwriteTable_Schema_CompositePartitionKey {
            enforcement_in_record: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_Schema_CompositePartitionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTable_Schema_CompositePartitionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreamwriteTable_Schema_CompositePartitionKeyRef {
        TimestreamwriteTable_Schema_CompositePartitionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_Schema_CompositePartitionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforcement_in_record` after provisioning.\n"]
    pub fn enforcement_in_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_in_record", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreamwriteTable_SchemaDynamic {
    composite_partition_key: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreamwriteTable_Schema_CompositePartitionKey>>,
}

#[derive(Serialize)]
pub struct TimestreamwriteTable_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composite_partition_key: std::option::Option<std::vec::Vec<TimestreamwriteTable_Schema_CompositePartitionKey>>,
    dynamic: TimestreamwriteTable_SchemaDynamic,
}

impl TimestreamwriteTable_Schema {
    #[doc = "Set the field `composite_partition_key`.\n"]
    pub fn set_composite_partition_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreamwriteTable_Schema_CompositePartitionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.composite_partition_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.composite_partition_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreamwriteTable_Schema {
    type O = terrars::BlockAssignable<TimestreamwriteTable_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreamwriteTable_Schema {}

impl BuildTimestreamwriteTable_Schema {
    pub fn build(self) -> TimestreamwriteTable_Schema {
        TimestreamwriteTable_Schema {
            composite_partition_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreamwriteTable_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreamwriteTable_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TimestreamwriteTable_SchemaRef {
        TimestreamwriteTable_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreamwriteTable_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `composite_partition_key` after provisioning.\n"]
    pub fn composite_partition_key(&self) -> terrars::ListRef<TimestreamwriteTable_Schema_CompositePartitionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.composite_partition_key", self.base))
    }
}
