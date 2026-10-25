#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3tablesTableBucketData_ {
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
    encryption_configuration: std::option::Option<S3tablesTableBucket_EncryptionConfiguration>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_configuration: std::option::Option<S3tablesTableBucket_MaintenanceConfiguration>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct S3tablesTableBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3tablesTableBucketData_>,
}

#[derive(Clone)]
pub struct S3tablesTableBucket(std::rc::Rc<S3tablesTableBucket_>);

impl S3tablesTableBucket {
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
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<S3tablesTableBucket_EncryptionConfiguration>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_configuration`.\n"]
    pub fn set_maintenance_configuration(
        self,
        v: impl std::convert::Into<S3tablesTableBucket_MaintenanceConfiguration>,
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableBucket_EncryptionConfigurationRef {
        S3tablesTableBucket_EncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableBucket_MaintenanceConfigurationRef {
        S3tablesTableBucket_MaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for S3tablesTableBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3tablesTableBucket { }

impl terrars::ToListMappable for S3tablesTableBucket {
    type O = terrars::ListRef<S3tablesTableBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3tablesTableBucket_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3tables_table_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3tablesTableBucket {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3tablesTableBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> S3tablesTableBucket {
        let out = S3tablesTableBucket(std::rc::Rc::new(S3tablesTableBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3tablesTableBucketData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                maintenance_configuration: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3tablesTableBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3tablesTableBucketRef {
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> S3tablesTableBucket_EncryptionConfigurationRef {
        S3tablesTableBucket_EncryptionConfigurationRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_configuration` after provisioning.\n"]
    pub fn maintenance_configuration(&self) -> S3tablesTableBucket_MaintenanceConfigurationRef {
        S3tablesTableBucket_MaintenanceConfigurationRef::new(
            self.shared().clone(),
            format!("{}.maintenance_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3tablesTableBucket_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3tablesTableBucket_EncryptionConfiguration {
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

impl terrars::ToListMappable for S3tablesTableBucket_EncryptionConfiguration {
    type O = terrars::BlockAssignable<S3tablesTableBucket_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTableBucket_EncryptionConfiguration {}

impl BuildS3tablesTableBucket_EncryptionConfiguration {
    pub fn build(self) -> S3tablesTableBucket_EncryptionConfiguration {
        S3tablesTableBucket_EncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTableBucket_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableBucket_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTableBucket_EncryptionConfigurationRef {
        S3tablesTableBucket_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTableBucket_EncryptionConfigurationRef {
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
pub struct S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_current_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unreferenced_days: std::option::Option<terrars::PrimField<f64>>,
}

impl S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
    #[doc = "Set the field `non_current_days`.\n"]
    pub fn set_non_current_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.non_current_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unreferenced_days`.\n"]
    pub fn set_unreferenced_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unreferenced_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
    type O =
        terrars
        ::BlockAssignable<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {}

impl BuildS3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
    pub fn build(self) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings {
            non_current_days: core::default::Default::default(),
            unreferenced_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `non_current_days` after provisioning.\n"]
    pub fn non_current_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_current_days", self.base))
    }

    #[doc = "Get a reference to the value of field `unreferenced_days` after provisioning.\n"]
    pub fn unreferenced_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unreferenced_days", self.base))
    }
}

#[derive(Serialize)]
pub struct S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_Settings>,
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

impl terrars::ToListMappable for S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
    type O = terrars::BlockAssignable<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {}

impl BuildS3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
    pub fn build(self) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval {
            settings: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval_SettingsRef::new(
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
pub struct S3tablesTableBucket_MaintenanceConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_unreferenced_file_removal: std
    ::option
    ::Option<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval>,
}

impl S3tablesTableBucket_MaintenanceConfiguration {
    #[doc = "Set the field `iceberg_unreferenced_file_removal`.\n"]
    pub fn set_iceberg_unreferenced_file_removal(
        mut self,
        v: impl std::convert::Into<S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemoval>,
    ) -> Self {
        self.iceberg_unreferenced_file_removal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3tablesTableBucket_MaintenanceConfiguration {
    type O = terrars::BlockAssignable<S3tablesTableBucket_MaintenanceConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3tablesTableBucket_MaintenanceConfiguration {}

impl BuildS3tablesTableBucket_MaintenanceConfiguration {
    pub fn build(self) -> S3tablesTableBucket_MaintenanceConfiguration {
        S3tablesTableBucket_MaintenanceConfiguration {
            iceberg_unreferenced_file_removal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3tablesTableBucket_MaintenanceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3tablesTableBucket_MaintenanceConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3tablesTableBucket_MaintenanceConfigurationRef {
        S3tablesTableBucket_MaintenanceConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3tablesTableBucket_MaintenanceConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_unreferenced_file_removal` after provisioning.\n"]
    pub fn iceberg_unreferenced_file_removal(
        &self,
    ) -> S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef {
        S3tablesTableBucket_MaintenanceConfiguration_IcebergUnreferencedFileRemovalRef::new(
            self.shared().clone(),
            format!("{}.iceberg_unreferenced_file_removal", self.base),
        )
    }
}
