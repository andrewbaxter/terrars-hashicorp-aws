use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCurReportDefinitionData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    report_name: terrars::PrimField<std::string::String>,
}

struct DataCurReportDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCurReportDefinitionData>,
}

#[derive(Clone)]
pub struct DataCurReportDefinition(std::rc::Rc<DataCurReportDefinition_>);

impl DataCurReportDefinition {
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

    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_schema_elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_closed_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}

impl terrars::Referable for DataCurReportDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCurReportDefinition { }

impl terrars::ToListMappable for DataCurReportDefinition {
    type O = terrars::ListRef<DataCurReportDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCurReportDefinition_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cur_report_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCurReportDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub report_name: terrars::PrimField<std::string::String>,
}

impl BuildDataCurReportDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCurReportDefinition {
        let out = DataCurReportDefinition(std::rc::Rc::new(DataCurReportDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCurReportDefinitionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                report_name: self.report_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCurReportDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCurReportDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCurReportDefinitionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `additional_artifacts` after provisioning.\n"]
    pub fn additional_artifacts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_schema_elements` after provisioning.\n"]
    pub fn additional_schema_elements(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_schema_elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_closed_reports` after provisioning.\n"]
    pub fn refresh_closed_reports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_closed_reports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_name` after provisioning.\n"]
    pub fn report_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_versioning` after provisioning.\n"]
    pub fn report_versioning(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }
}
