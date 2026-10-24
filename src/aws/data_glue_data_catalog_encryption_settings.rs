use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueDataCatalogEncryptionSettingsData {
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
}

struct DataGlueDataCatalogEncryptionSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlueDataCatalogEncryptionSettingsData>,
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

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataGlueDataCatalogEncryptionSettingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: self.catalog_id,
                id: core::default::Default::default(),
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
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_connection_password_encrypted: std::option::Option<terrars::PrimField<bool>>,
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
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
::ToListMappable for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    type O =
        terrars
        ::BlockAssignable<
            DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {}

impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    pub fn build(
        self,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
            aws_kms_key_id: core::default::Default::default(),
            return_connection_password_encrypted: core::default::Default::default(),
        }
    }
}

pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
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
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = "Set the field `catalog_encryption_mode`.\n"]
    pub fn set_catalog_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.catalog_encryption_mode = std::option::Option::Some(v.into());
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

impl terrars
::ToListMappable for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    type O =
        terrars
        ::BlockAssignable<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {}

impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    pub fn build(self) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
            catalog_encryption_mode: core::default::Default::default(),
            sse_aws_kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_encryption_mode` after provisioning.\n"]
    pub fn catalog_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_aws_kms_key_id` after provisioning.\n"]
    pub fn sse_aws_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sse_aws_kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_password_encryption: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest: std
    ::option
    ::Option<
        terrars::ListField<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>,
    >,
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
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
                            DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
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
                            DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl,
                        >,
                    >,
    ) -> Self {
        self.encryption_at_rest = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    type O = terrars::BlockAssignable<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {}

impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    pub fn build(self) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
            connection_password_encryption: core::default::Default::default(),
            encryption_at_rest: core::default::Default::default(),
        }
    }
}

pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_password_encryption` after provisioning.\n"]
    pub fn connection_password_encryption(
        &self,
    ) -> terrars
    ::ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_password_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(
        &self,
    ) -> terrars::ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.base))
    }
}
