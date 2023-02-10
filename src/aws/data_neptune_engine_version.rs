use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNeptuneEngineVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_group_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_versions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

struct DataNeptuneEngineVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNeptuneEngineVersionData>,
}

#[derive(Clone)]
pub struct DataNeptuneEngineVersion(Rc<DataNeptuneEngineVersion_>);

impl DataNeptuneEngineVersion {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `parameter_group_family`.\n"]
    pub fn set_parameter_group_family(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_family = Some(v.into());
        self
    }

    #[doc= "Set the field `preferred_versions`.\n"]
    pub fn set_preferred_versions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_versions = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exportable_log_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_timezones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_read_replica", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.valid_upgrade_targets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }
}

impl Referable for DataNeptuneEngineVersion {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataNeptuneEngineVersion { }

impl ToListMappable for DataNeptuneEngineVersion {
    type O = ListRef<DataNeptuneEngineVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNeptuneEngineVersion_ {
    fn extract_datasource_type(&self) -> String {
        "aws_neptune_engine_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNeptuneEngineVersion {
    pub tf_id: String,
}

impl BuildDataNeptuneEngineVersion {
    pub fn build(self, stack: &mut Stack) -> DataNeptuneEngineVersion {
        let out = DataNeptuneEngineVersion(Rc::new(DataNeptuneEngineVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNeptuneEngineVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataNeptuneEngineVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNeptuneEngineVersionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exportable_log_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_timezones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_read_replica", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.valid_upgrade_targets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_description", self.extract_ref()))
    }
}
