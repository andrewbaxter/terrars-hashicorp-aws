use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAcmCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statuses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}

struct DataAcmCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAcmCertificateData>,
}

#[derive(Clone)]
pub struct DataAcmCertificate(Rc<DataAcmCertificate_>);

impl DataAcmCertificate {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `key_types`.\n"]
    pub fn set_key_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().key_types = Some(v.into());
        self
    }

    #[doc= "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = Some(v.into());
        self
    }

    #[doc= "Set the field `statuses`.\n"]
    pub fn set_statuses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().statuses = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `types`.\n"]
    pub fn set_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().types = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_types` after provisioning.\n"]
    pub fn key_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.key_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statuses` after provisioning.\n"]
    pub fn statuses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.statuses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.extract_ref()))
    }
}

impl Referable for DataAcmCertificate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAcmCertificate { }

impl ToListMappable for DataAcmCertificate {
    type O = ListRef<DataAcmCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAcmCertificate_ {
    fn extract_datasource_type(&self) -> String {
        "aws_acm_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAcmCertificate {
    pub tf_id: String,
    #[doc= ""]
    pub domain: PrimField<String>,
}

impl BuildDataAcmCertificate {
    pub fn build(self, stack: &mut Stack) -> DataAcmCertificate {
        let out = DataAcmCertificate(Rc::new(DataAcmCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAcmCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                domain: self.domain,
                id: core::default::Default::default(),
                key_types: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                statuses: core::default::Default::default(),
                tags: core::default::Default::default(),
                types: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAcmCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAcmCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAcmCertificateRef {
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

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_types` after provisioning.\n"]
    pub fn key_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.key_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statuses` after provisioning.\n"]
    pub fn statuses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.statuses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.extract_ref()))
    }
}
