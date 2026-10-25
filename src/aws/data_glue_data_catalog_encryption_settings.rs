#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueDataCatalogEncryptionSettingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    catalog_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataGlueDataCatalogEncryptionSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlueDataCatalogEncryptionSettingsData_>,
}

#[derive(Clone)]
pub struct DataGlueDataCatalogEncryptionSettings(std::rc::Rc<DataGlueDataCatalogEncryptionSettings_>);

impl DataGlueDataCatalogEncryptionSettings {
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

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
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
}

impl terrars::Referable for DataGlueDataCatalogEncryptionSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGlueDataCatalogEncryptionSettings { }

impl terrars::ToListMappable for DataGlueDataCatalogEncryptionSettings {
    type O = terrars::ListRef<DataGlueDataCatalogEncryptionSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGlueDataCatalogEncryptionSettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_glue_data_catalog_encryption_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub catalog_id: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueDataCatalogEncryptionSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGlueDataCatalogEncryptionSettings {
        let out = DataGlueDataCatalogEncryptionSettings(std::rc::Rc::new(DataGlueDataCatalogEncryptionSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGlueDataCatalogEncryptionSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: self.catalog_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlueDataCatalogEncryptionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueDataCatalogEncryptionSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGlueDataCatalogEncryptionSettingsRef {
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

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
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
}

#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_connection_password_encrypted: std::option::Option<terrars::PrimField<bool>>,
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
    #[doc = "Set the field `aws_kms_key_id`.\n"]
    pub fn set_aws_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_connection_password_encrypted`.\n"]
    pub fn set_return_connection_password_encrypted(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.return_connection_password_encrypted = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
    type O =
        terrars
        ::BlockAssignable<
            DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {}

impl BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
    pub fn build(
        self,
    ) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
            aws_kms_key_id: core::default::Default::default(),
            return_connection_password_encrypted: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_kms_key_id` after provisioning.\n"]
    pub fn aws_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `return_connection_password_encrypted` after provisioning.\n"]
    pub fn return_connection_password_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_connection_password_encrypted", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_encryption_service_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
    #[doc = "Set the field `catalog_encryption_mode`.\n"]
    pub fn set_catalog_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.catalog_encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog_encryption_service_role`.\n"]
    pub fn set_catalog_encryption_service_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.catalog_encryption_service_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sse_aws_kms_key_id`.\n"]
    pub fn set_sse_aws_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sse_aws_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
    type O =
        terrars
        ::BlockAssignable<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {}

impl BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
    pub fn build(self) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest {
            catalog_encryption_mode: core::default::Default::default(),
            catalog_encryption_service_role: core::default::Default::default(),
            sse_aws_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_encryption_mode` after provisioning.\n"]
    pub fn catalog_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog_encryption_service_role` after provisioning.\n"]
    pub fn catalog_encryption_service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_encryption_service_role", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_aws_kms_key_id` after provisioning.\n"]
    pub fn sse_aws_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sse_aws_kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_password_encryption: std
    ::option
    ::Option<
        terrars
        ::ListField<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest: std
    ::option
    ::Option<
        terrars::ListField<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest>,
    >,
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
    #[doc = "Set the field `connection_password_encryption`.\n"]
    pub fn set_connection_password_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryption,
                        >,
                    >,
    ) -> Self {
        self.connection_password_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_at_rest`.\n"]
    pub fn set_encryption_at_rest(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRest,
                        >,
                    >,
    ) -> Self {
        self.encryption_at_rest = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
    type O = terrars::BlockAssignable<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {}

impl BuildDataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
    pub fn build(self) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
            connection_password_encryption: core::default::Default::default(),
            encryption_at_rest: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef {
        DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_password_encryption` after provisioning.\n"]
    pub fn connection_password_encryption(
        &self,
    ) -> terrars
    ::ListRef<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_ConnectionPasswordEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_password_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(
        &self,
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettings_DataCatalogEncryptionSettings_EncryptionAtRestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.base))
    }
}
