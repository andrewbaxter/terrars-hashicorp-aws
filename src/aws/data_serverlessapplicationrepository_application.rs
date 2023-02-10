use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServerlessapplicationrepositoryApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_version: Option<PrimField<String>>,
}

struct DataServerlessapplicationrepositoryApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServerlessapplicationrepositoryApplicationData>,
}

#[derive(Clone)]
pub struct DataServerlessapplicationrepositoryApplication(Rc<DataServerlessapplicationrepositoryApplication_>);

impl DataServerlessapplicationrepositoryApplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `semantic_version`.\n"]
    pub fn set_semantic_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().semantic_version = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_capabilities` after provisioning.\n"]
    pub fn required_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.required_capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `semantic_version` after provisioning.\n"]
    pub fn semantic_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.semantic_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_code_url` after provisioning.\n"]
    pub fn source_code_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_code_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }
}

impl Datasource for DataServerlessapplicationrepositoryApplication {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataServerlessapplicationrepositoryApplication {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataServerlessapplicationrepositoryApplication {
    type O = ListRef<DataServerlessapplicationrepositoryApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataServerlessapplicationrepositoryApplication_ {
    fn extract_datasource_type(&self) -> String {
        "aws_serverlessapplicationrepository_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServerlessapplicationrepositoryApplication {
    pub tf_id: String,
    #[doc= ""]
    pub application_id: PrimField<String>,
}

impl BuildDataServerlessapplicationrepositoryApplication {
    pub fn build(self, stack: &mut Stack) -> DataServerlessapplicationrepositoryApplication {
        let out =
            DataServerlessapplicationrepositoryApplication(Rc::new(DataServerlessapplicationrepositoryApplication_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataServerlessapplicationrepositoryApplicationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    application_id: self.application_id,
                    id: core::default::Default::default(),
                    semantic_version: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServerlessapplicationrepositoryApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServerlessapplicationrepositoryApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServerlessapplicationrepositoryApplicationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_capabilities` after provisioning.\n"]
    pub fn required_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.required_capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `semantic_version` after provisioning.\n"]
    pub fn semantic_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.semantic_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_code_url` after provisioning.\n"]
    pub fn source_code_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_code_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }
}
