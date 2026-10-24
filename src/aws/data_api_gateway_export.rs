use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataApiGatewayExportData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepts: std::option::Option<terrars::PrimField<std::string::String>>,
    export_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    rest_api_id: terrars::PrimField<std::string::String>,
    stage_name: terrars::PrimField<std::string::String>,
}

struct DataApiGatewayExport_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApiGatewayExportData>,
}

#[derive(Clone)]
pub struct DataApiGatewayExport(std::rc::Rc<DataApiGatewayExport_>);

impl DataApiGatewayExport {
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

    #[doc = "Set the field `accepts`.\n"]
    pub fn set_accepts(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().accepts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accepts` after provisioning.\n"]
    pub fn accepts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accepts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\n"]
    pub fn content_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_type` after provisioning.\n"]
    pub fn export_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataApiGatewayExport {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApiGatewayExport { }

impl terrars::ToListMappable for DataApiGatewayExport {
    type O = terrars::ListRef<DataApiGatewayExportRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApiGatewayExport_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_api_gateway_export".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiGatewayExport {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub export_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rest_api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage_name: terrars::PrimField<std::string::String>,
}

impl BuildDataApiGatewayExport {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApiGatewayExport {
        let out = DataApiGatewayExport(std::rc::Rc::new(DataApiGatewayExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApiGatewayExportData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                accepts: core::default::Default::default(),
                export_type: self.export_type,
                id: core::default::Default::default(),
                parameters: core::default::Default::default(),
                rest_api_id: self.rest_api_id,
                stage_name: self.stage_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiGatewayExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiGatewayExportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiGatewayExportRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accepts` after provisioning.\n"]
    pub fn accepts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accepts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_disposition` after provisioning.\n"]
    pub fn content_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_type` after provisioning.\n"]
    pub fn export_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}
