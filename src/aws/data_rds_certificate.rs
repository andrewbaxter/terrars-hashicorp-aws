use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRdsCertificateData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_valid_till: Option<PrimField<bool>>,
}

struct DataRdsCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRdsCertificateData>,
}

#[derive(Clone)]
pub struct DataRdsCertificate(Rc<DataRdsCertificate_>);

impl DataRdsCertificate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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

    #[doc= "Set the field `latest_valid_till`.\n"]
    pub fn set_latest_valid_till(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().latest_valid_till = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_type` after provisioning.\n"]
    pub fn certificate_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_override` after provisioning.\n"]
    pub fn customer_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_override", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_override_valid_till` after provisioning.\n"]
    pub fn customer_override_valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_override_valid_till", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latest_valid_till` after provisioning.\n"]
    pub fn latest_valid_till(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest_valid_till", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `thumbprint` after provisioning.\n"]
    pub fn thumbprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_till` after provisioning.\n"]
    pub fn valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_till", self.extract_ref()))
    }
}

impl Datasource for DataRdsCertificate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataRdsCertificate {
    type O = ListRef<DataRdsCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRdsCertificate_ {
    fn extract_datasource_type(&self) -> String {
        "aws_rds_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRdsCertificate {
    pub tf_id: String,
}

impl BuildDataRdsCertificate {
    pub fn build(self, stack: &mut Stack) -> DataRdsCertificate {
        let out = DataRdsCertificate(Rc::new(DataRdsCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRdsCertificateData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                latest_valid_till: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRdsCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRdsCertificateRef {
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

    #[doc= "Get a reference to the value of field `certificate_type` after provisioning.\n"]
    pub fn certificate_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_override` after provisioning.\n"]
    pub fn customer_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_override", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_override_valid_till` after provisioning.\n"]
    pub fn customer_override_valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_override_valid_till", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latest_valid_till` after provisioning.\n"]
    pub fn latest_valid_till(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest_valid_till", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `thumbprint` after provisioning.\n"]
    pub fn thumbprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thumbprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_till` after provisioning.\n"]
    pub fn valid_till(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_till", self.extract_ref()))
    }
}
