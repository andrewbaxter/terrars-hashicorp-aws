use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontLogDeliveryCanonicalUserIdData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataCloudfrontLogDeliveryCanonicalUserId_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontLogDeliveryCanonicalUserIdData>,
}

#[derive(Clone)]
pub struct DataCloudfrontLogDeliveryCanonicalUserId(Rc<DataCloudfrontLogDeliveryCanonicalUserId_>);

impl DataCloudfrontLogDeliveryCanonicalUserId {
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

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Datasource for DataCloudfrontLogDeliveryCanonicalUserId {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataCloudfrontLogDeliveryCanonicalUserId {
    type O = ListRef<DataCloudfrontLogDeliveryCanonicalUserIdRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudfrontLogDeliveryCanonicalUserId_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_log_delivery_canonical_user_id".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontLogDeliveryCanonicalUserId {
    pub tf_id: String,
}

impl BuildDataCloudfrontLogDeliveryCanonicalUserId {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontLogDeliveryCanonicalUserId {
        let out = DataCloudfrontLogDeliveryCanonicalUserId(Rc::new(DataCloudfrontLogDeliveryCanonicalUserId_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfrontLogDeliveryCanonicalUserIdData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontLogDeliveryCanonicalUserIdRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontLogDeliveryCanonicalUserIdRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontLogDeliveryCanonicalUserIdRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
