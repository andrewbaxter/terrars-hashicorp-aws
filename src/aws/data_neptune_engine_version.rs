use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNeptuneEngineVersionData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_group_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataNeptuneEngineVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNeptuneEngineVersionData>,
}

#[derive(Clone)]
pub struct DataNeptuneEngineVersion(std::rc::Rc<DataNeptuneEngineVersion_>);

impl DataNeptuneEngineVersion {
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

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_group_family`.\n"]
    pub fn set_parameter_group_family(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().parameter_group_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_versions`.\n"]
    pub fn set_preferred_versions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exportable_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_timezones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_read_replica", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.valid_upgrade_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }
}

impl terrars::Referable for DataNeptuneEngineVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNeptuneEngineVersion { }

impl terrars::ToListMappable for DataNeptuneEngineVersion {
    type O = terrars::ListRef<DataNeptuneEngineVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNeptuneEngineVersion_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_neptune_engine_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNeptuneEngineVersion {
    pub tf_id: std::string::String,
}

impl BuildDataNeptuneEngineVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNeptuneEngineVersion {
        let out = DataNeptuneEngineVersion(std::rc::Rc::new(DataNeptuneEngineVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNeptuneEngineVersionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                engine: core::default::Default::default(),
                id: core::default::Default::default(),
                parameter_group_family: core::default::Default::default(),
                preferred_versions: core::default::Default::default(),
                version: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNeptuneEngineVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNeptuneEngineVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNeptuneEngineVersionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exportable_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_timezones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_read_replica", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.valid_upgrade_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }
}
