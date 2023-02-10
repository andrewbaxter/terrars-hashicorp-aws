use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamSamlProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataIamSamlProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamSamlProviderData>,
}

#[derive(Clone)]
pub struct DataIamSamlProvider(Rc<DataIamSamlProvider_>);

impl DataIamSamlProvider {
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

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_metadata_document` after provisioning.\n"]
    pub fn saml_metadata_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_metadata_document", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }
}

impl Datasource for DataIamSamlProvider {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataIamSamlProvider {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataIamSamlProvider {
    type O = ListRef<DataIamSamlProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataIamSamlProvider_ {
    fn extract_datasource_type(&self) -> String {
        "aws_iam_saml_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamSamlProvider {
    pub tf_id: String,
    #[doc= ""]
    pub arn: PrimField<String>,
}

impl BuildDataIamSamlProvider {
    pub fn build(self, stack: &mut Stack) -> DataIamSamlProvider {
        let out = DataIamSamlProvider(Rc::new(DataIamSamlProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamSamlProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamSamlProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIamSamlProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamSamlProviderRef {
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

    #[doc= "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_metadata_document` after provisioning.\n"]
    pub fn saml_metadata_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_metadata_document", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }
}
