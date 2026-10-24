use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueDataCatalogEncryptionSettingsData {
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
    data_catalog_encryption_settings: std
    ::option
    ::Option<std::vec::Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>>,
    dynamic: GlueDataCatalogEncryptionSettingsDynamic,
}

struct GlueDataCatalogEncryptionSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueDataCatalogEncryptionSettingsData>,
}

#[derive(Clone)]
pub struct GlueDataCatalogEncryptionSettings(std::rc::Rc<GlueDataCatalogEncryptionSettings_>);

impl GlueDataCatalogEncryptionSettings {
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

    #[doc = "Set the field `data_catalog_encryption_settings`.\n"]
    pub fn set_data_catalog_encryption_settings(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_catalog_encryption_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_catalog_encryption_settings = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> terrars::ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GlueDataCatalogEncryptionSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueDataCatalogEncryptionSettings { }

impl terrars::ToListMappable for GlueDataCatalogEncryptionSettings {
    type O = terrars::ListRef<GlueDataCatalogEncryptionSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueDataCatalogEncryptionSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_data_catalog_encryption_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueDataCatalogEncryptionSettings {
    pub tf_id: std::string::String,
}

impl BuildGlueDataCatalogEncryptionSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueDataCatalogEncryptionSettings {
        let out = GlueDataCatalogEncryptionSettings(std::rc::Rc::new(GlueDataCatalogEncryptionSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueDataCatalogEncryptionSettingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                id: core::default::Default::default(),
                data_catalog_encryption_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueDataCatalogEncryptionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueDataCatalogEncryptionSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueDataCatalogEncryptionSettingsRef {
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

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> terrars::ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    return_connection_password_encrypted: terrars::PrimField<bool>,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[doc = "Set the field `aws_kms_key_id`.\n"]
    pub fn set_aws_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    type O =
        terrars
        ::BlockAssignable<
            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[doc = ""]
    pub return_connection_password_encrypted: terrars::PrimField<bool>,
}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    pub fn build(
        self,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
            aws_kms_key_id: core::default::Default::default(),
            return_connection_password_encrypted: self.return_connection_password_encrypted,
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
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
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    catalog_encryption_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_aws_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = "Set the field `sse_aws_kms_key_id`.\n"]
    pub fn set_sse_aws_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sse_aws_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    type O =
        terrars::BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = ""]
    pub catalog_encryption_mode: terrars::PrimField<std::string::String>,
}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    pub fn build(self) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
            catalog_encryption_mode: self.catalog_encryption_mode,
            sse_aws_kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
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

#[derive(Serialize, Default)]
struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElDynamic {
    connection_password_encryption: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl>,
    >,
    encryption_at_rest: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>,
    >,
}

#[derive(Serialize)]
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_password_encryption: std
    ::option
    ::Option<
        std::vec::Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_at_rest: std
    ::option
    ::Option<std::vec::Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>>,
    dynamic: GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElDynamic,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[doc = "Set the field `connection_password_encryption`.\n"]
    pub fn set_connection_password_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_password_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_password_encryption = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_at_rest = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_at_rest = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    type O = terrars::BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    pub fn build(self) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
            connection_password_encryption: core::default::Default::default(),
            encryption_at_rest: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_password_encryption` after provisioning.\n"]
    pub fn connection_password_encryption(
        &self,
    ) -> terrars
    ::ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_password_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(
        &self,
    ) -> terrars::ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueDataCatalogEncryptionSettingsDynamic {
    data_catalog_encryption_settings: std
    ::option
    ::Option<terrars::DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>>,
}
