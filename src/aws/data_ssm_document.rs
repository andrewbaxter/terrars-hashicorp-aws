use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmDocumentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataSsmDocument_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmDocumentData>,
}

#[derive(Clone)]
pub struct DataSsmDocument(Rc<DataSsmDocument_>);

impl DataSsmDocument {
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

    #[doc= "Set the field `document_format`.\n"]
    pub fn set_document_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_format = Some(v.into());
        self
    }

    #[doc= "Set the field `document_version`.\n"]
    pub fn set_document_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_version = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_format` after provisioning.\n"]
    pub fn document_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_format", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Datasource for DataSsmDocument {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataSsmDocument {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataSsmDocument {
    type O = ListRef<DataSsmDocumentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmDocument_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssm_document".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmDocument {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataSsmDocument {
    pub fn build(self, stack: &mut Stack) -> DataSsmDocument {
        let out = DataSsmDocument(Rc::new(DataSsmDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmDocumentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                document_format: core::default::Default::default(),
                document_version: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmDocumentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmDocumentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmDocumentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_format` after provisioning.\n"]
    pub fn document_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_format", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}
