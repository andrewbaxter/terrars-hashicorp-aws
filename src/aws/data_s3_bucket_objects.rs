use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3BucketObjectsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetch_owner: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_keys: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_after: Option<PrimField<String>>,
}

struct DataS3BucketObjects_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataS3BucketObjectsData>,
}

#[derive(Clone)]
pub struct DataS3BucketObjects(Rc<DataS3BucketObjects_>);

impl DataS3BucketObjects {
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

    #[doc= "Set the field `delimiter`.\n"]
    pub fn set_delimiter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delimiter = Some(v.into());
        self
    }

    #[doc= "Set the field `encoding_type`.\n"]
    pub fn set_encoding_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encoding_type = Some(v.into());
        self
    }

    #[doc= "Set the field `fetch_owner`.\n"]
    pub fn set_fetch_owner(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fetch_owner = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `max_keys`.\n"]
    pub fn set_max_keys(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_keys = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `start_after`.\n"]
    pub fn set_start_after(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_after = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `common_prefixes` after provisioning.\n"]
    pub fn common_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.common_prefixes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fetch_owner` after provisioning.\n"]
    pub fn fetch_owner(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fetch_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_keys` after provisioning.\n"]
    pub fn max_keys(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owners` after provisioning.\n"]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.owners", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_after` after provisioning.\n"]
    pub fn start_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_after", self.extract_ref()))
    }
}

impl Datasource for DataS3BucketObjects {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataS3BucketObjects {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataS3BucketObjects {
    type O = ListRef<DataS3BucketObjectsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataS3BucketObjects_ {
    fn extract_datasource_type(&self) -> String {
        "aws_s3_bucket_objects".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3BucketObjects {
    pub tf_id: String,
    #[doc= ""]
    pub bucket: PrimField<String>,
}

impl BuildDataS3BucketObjects {
    pub fn build(self, stack: &mut Stack) -> DataS3BucketObjects {
        let out = DataS3BucketObjects(Rc::new(DataS3BucketObjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataS3BucketObjectsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket: self.bucket,
                delimiter: core::default::Default::default(),
                encoding_type: core::default::Default::default(),
                fetch_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                max_keys: core::default::Default::default(),
                prefix: core::default::Default::default(),
                start_after: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3BucketObjectsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3BucketObjectsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3BucketObjectsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `common_prefixes` after provisioning.\n"]
    pub fn common_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.common_prefixes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fetch_owner` after provisioning.\n"]
    pub fn fetch_owner(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fetch_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_keys` after provisioning.\n"]
    pub fn max_keys(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owners` after provisioning.\n"]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.owners", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_after` after provisioning.\n"]
    pub fn start_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_after", self.extract_ref()))
    }
}
