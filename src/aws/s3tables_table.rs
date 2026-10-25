#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3tablesTableData_ {
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
    encryption_configuration: std::option::Option<S3tablesTable_EncryptionConfiguration>,
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_configuration: std::option::Option<S3tablesTable_MaintenanceConfiguration>,
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<S3tablesTable_Metadata>>,
    dynamic: S3tablesTableDynamic,
}

struct S3tablesTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3tablesTableData_>,
}

#[derive(Clone)]
pub struct S3tablesTable(std::rc::Rc<S3tablesTable_>);

impl S3tablesTable {
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(self, v: impl std::convert::Into<S3tablesTable_EncryptionConfiguration>) -> Self {
        self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_configuration`.\n"]
    pub fn set_maintenance_configuration(
        self,
        v: impl std::convert::Into<S3tablesTable_MaintenanceConfiguration>,
    ) -> Self {
        self.0.data.borrow_mut().maintenance_configuration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl std::convert::Into<terrars::BlockAssignable<S3tablesTable_Metadata>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTable_EncryptionConfigurationRef {
        S3tablesTable_EncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTable_MaintenanceConfigurationRef {
        S3tablesTable_MaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_location` after provisioning.\n"]
    pub fn metadata_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_token` after provisioning.\n"]
    pub fn version_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warehouse_location` after provisioning.\n"]
    pub fn warehouse_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<S3tablesTable_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }
}

impl terrars::Referable for S3tablesTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3tablesTable { }

impl terrars::ToListMappable for S3tablesTable {
    type O = terrars::ListRef<S3tablesTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3tablesTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3tables_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3tablesTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_bucket_arn: terrars::PrimField<std::string::String>,
}

impl BuildS3tablesTable {
    pub fn build(self, stack: &mut terrars::Stack) -> S3tablesTable {
        let out = S3tablesTable(std::rc::Rc::new(S3tablesTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3tablesTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                format: self.format,
                maintenance_configuration: core::default::Default::default(),
                name: self.name,
                namespace: self.namespace,
                region: core::default::Default::default(),
                table_bucket_arn: self.table_bucket_arn,
                tags: core::default::Default::default(),
                metadata: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3tablesTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3tablesTableRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTable_EncryptionConfigurationRef {
        S3tablesTable_EncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTable_MaintenanceConfigurationRef {
        S3tablesTable_MaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_location` after provisioning.\n"]
    pub fn metadata_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_token` after provisioning.\n"]
    pub fn version_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warehouse_location` after provisioning.\n"]
    pub fn warehouse_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<S3tablesTable_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3tablesTableDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<S3tablesTable_Metadata>>,
}

#[derive(Serialize)]
pub struct S3tablesTable_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3tablesTable_EncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sse_algorithm`.\n"]
    pub fn set_sse_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sse_algorithm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_EncryptionConfiguration {
    type O = terrars::BlockAssignable<S3tablesTable_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_EncryptionConfiguration {}

impl BuildS3tablesTable_EncryptionConfiguration {
    pub fn build(self) -> S3tablesTable_EncryptionConfiguration {
        S3tablesTable_EncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3tablesTable_EncryptionConfigurationRef {
        S3tablesTable_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_file_size_mb: std::option::Option<terrars::PrimField<f64>>,
}

impl S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
    #[doc = "Set the field `target_file_size_mb`.\n"]
    pub fn set_target_file_size_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_file_size_mb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
    type O = terrars::BlockAssignable<S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {}

impl BuildS3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
    pub fn build(self) -> S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
        S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings {
            target_file_size_mb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
        S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_file_size_mb` after provisioning.\n"]
    pub fn target_file_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_file_size_mb", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergCompaction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3tablesTable_MaintenanceConfiguration_IcebergCompaction {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<S3tablesTable_MaintenanceConfiguration_IcebergCompaction_Settings>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_MaintenanceConfiguration_IcebergCompaction {
    type O = terrars::BlockAssignable<S3tablesTable_MaintenanceConfiguration_IcebergCompaction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_MaintenanceConfiguration_IcebergCompaction {}

impl BuildS3tablesTable_MaintenanceConfiguration_IcebergCompaction {
    pub fn build(self) -> S3tablesTable_MaintenanceConfiguration_IcebergCompaction {
        S3tablesTable_MaintenanceConfiguration_IcebergCompaction {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
        S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef {
        S3tablesTable_MaintenanceConfiguration_IcebergCompaction_SettingsRef::new(
            self.shared().clone(),
            format!("{}.settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_snapshot_age_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_snapshots_to_keep: std::option::Option<terrars::PrimField<f64>>,
}

impl S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
    #[doc = "Set the field `max_snapshot_age_hours`.\n"]
    pub fn set_max_snapshot_age_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_snapshot_age_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_snapshots_to_keep`.\n"]
    pub fn set_min_snapshots_to_keep(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_snapshots_to_keep = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
    type O = terrars::BlockAssignable<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {}

impl BuildS3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
    pub fn build(self) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings {
            max_snapshot_age_hours: core::default::Default::default(),
            min_snapshots_to_keep: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_snapshot_age_hours` after provisioning.\n"]
    pub fn max_snapshot_age_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_snapshot_age_hours", self.base))
    }

    #[doc = "Get a reference to the value of field `min_snapshots_to_keep` after provisioning.\n"]
    pub fn min_snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_snapshots_to_keep", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_Settings>,
    ) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
    type O = terrars::BlockAssignable<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {}

impl BuildS3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
    pub fn build(self) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement_SettingsRef::new(
            self.shared().clone(),
            format!("{}.settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_MaintenanceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_compaction: std::option::Option<S3tablesTable_MaintenanceConfiguration_IcebergCompaction>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_snapshot_management: std::option::Option<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement>,
}

impl S3tablesTable_MaintenanceConfiguration {
    #[doc = "Set the field `iceberg_compaction`.\n"]
    pub fn set_iceberg_compaction(
        mut self,
        v: impl std::convert::Into<S3tablesTable_MaintenanceConfiguration_IcebergCompaction>,
    ) -> Self {
        self.iceberg_compaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iceberg_snapshot_management`.\n"]
    pub fn set_iceberg_snapshot_management(
        mut self,
        v: impl std::convert::Into<S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagement>,
    ) -> Self {
        self.iceberg_snapshot_management = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_MaintenanceConfiguration {
    type O = terrars::BlockAssignable<S3tablesTable_MaintenanceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_MaintenanceConfiguration {}

impl BuildS3tablesTable_MaintenanceConfiguration {
    pub fn build(self) -> S3tablesTable_MaintenanceConfiguration {
        S3tablesTable_MaintenanceConfiguration {
            iceberg_compaction: core::default::Default::default(),
            iceberg_snapshot_management: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MaintenanceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MaintenanceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3tablesTable_MaintenanceConfigurationRef {
        S3tablesTable_MaintenanceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MaintenanceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_compaction` after provisioning.\n"]
    pub fn iceberg_compaction(&self) -> S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef {
        S3tablesTable_MaintenanceConfiguration_IcebergCompactionRef::new(
            self.shared().clone(),
            format!("{}.iceberg_compaction", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `iceberg_snapshot_management` after provisioning.\n"]
    pub fn iceberg_snapshot_management(&self) -> S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef {
        S3tablesTable_MaintenanceConfiguration_IcebergSnapshotManagementRef::new(
            self.shared().clone(),
            format!("{}.iceberg_snapshot_management", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct S3tablesTable_Metadata_Iceberg_Schema_Field {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl S3tablesTable_Metadata_Iceberg_Schema_Field {
    #[doc =
        "Set the field `required`.\nA Boolean value that specifies whether values are required for each row in this field. Default: false."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_Metadata_Iceberg_Schema_Field {
    type O = terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg_Schema_Field>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_Metadata_Iceberg_Schema_Field {
    #[doc = "The name of the field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The field type. S3 Tables supports all Apache Iceberg primitive types."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildS3tablesTable_Metadata_Iceberg_Schema_Field {
    pub fn build(self) -> S3tablesTable_Metadata_Iceberg_Schema_Field {
        S3tablesTable_Metadata_Iceberg_Schema_Field {
            name: self.name,
            required: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_Metadata_Iceberg_Schema_FieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_Metadata_Iceberg_Schema_FieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTable_Metadata_Iceberg_Schema_FieldRef {
        S3tablesTable_Metadata_Iceberg_Schema_FieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_Metadata_Iceberg_Schema_FieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nA Boolean value that specifies whether values are required for each row in this field. Default: false."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe field type. S3 Tables supports all Apache Iceberg primitive types."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3tablesTable_Metadata_Iceberg_SchemaDynamic {
    field: std::option::Option<terrars::DynamicBlock<S3tablesTable_Metadata_Iceberg_Schema_Field>>,
}

#[derive(Serialize)]
pub struct S3tablesTable_Metadata_Iceberg_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<std::vec::Vec<S3tablesTable_Metadata_Iceberg_Schema_Field>>,
    dynamic: S3tablesTable_Metadata_Iceberg_SchemaDynamic,
}

impl S3tablesTable_Metadata_Iceberg_Schema {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg_Schema_Field>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_Metadata_Iceberg_Schema {
    type O = terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_Metadata_Iceberg_Schema {}

impl BuildS3tablesTable_Metadata_Iceberg_Schema {
    pub fn build(self) -> S3tablesTable_Metadata_Iceberg_Schema {
        S3tablesTable_Metadata_Iceberg_Schema {
            field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_Metadata_Iceberg_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_Metadata_Iceberg_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3tablesTable_Metadata_Iceberg_SchemaRef {
        S3tablesTable_Metadata_Iceberg_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_Metadata_Iceberg_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::ListRef<S3tablesTable_Metadata_Iceberg_Schema_FieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3tablesTable_Metadata_IcebergDynamic {
    schema: std::option::Option<terrars::DynamicBlock<S3tablesTable_Metadata_Iceberg_Schema>>,
}

#[derive(Serialize)]
pub struct S3tablesTable_Metadata_Iceberg {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<S3tablesTable_Metadata_Iceberg_Schema>>,
    dynamic: S3tablesTable_Metadata_IcebergDynamic,
}

impl S3tablesTable_Metadata_Iceberg {
    #[doc =
        "Set the field `properties`.\nA map of configuration properties for the Iceberg table, for example `write.distribution-mode` and `write.sort-order`."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_Metadata_Iceberg {
    type O = terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_Metadata_Iceberg {}

impl BuildS3tablesTable_Metadata_Iceberg {
    pub fn build(self) -> S3tablesTable_Metadata_Iceberg {
        S3tablesTable_Metadata_Iceberg {
            properties: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_Metadata_IcebergRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_Metadata_IcebergRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3tablesTable_Metadata_IcebergRef {
        S3tablesTable_Metadata_IcebergRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_Metadata_IcebergRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA map of configuration properties for the Iceberg table, for example `write.distribution-mode` and `write.sort-order`."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<S3tablesTable_Metadata_Iceberg_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3tablesTable_MetadataDynamic {
    iceberg: std::option::Option<terrars::DynamicBlock<S3tablesTable_Metadata_Iceberg>>,
}

#[derive(Serialize)]
pub struct S3tablesTable_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg: std::option::Option<std::vec::Vec<S3tablesTable_Metadata_Iceberg>>,
    dynamic: S3tablesTable_MetadataDynamic,
}

impl S3tablesTable_Metadata {
    #[doc = "Set the field `iceberg`.\n"]
    pub fn set_iceberg(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3tablesTable_Metadata_Iceberg>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3tablesTable_Metadata {
    type O = terrars::BlockAssignable<S3tablesTable_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTable_Metadata {}

impl BuildS3tablesTable_Metadata {
    pub fn build(self) -> S3tablesTable_Metadata {
        S3tablesTable_Metadata {
            iceberg: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTable_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTable_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3tablesTable_MetadataRef {
        S3tablesTable_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTable_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg` after provisioning.\n"]
    pub fn iceberg(&self) -> terrars::ListRef<S3tablesTable_Metadata_IcebergRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg", self.base))
    }
}
