use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontOriginAccessIdentityData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
}

struct DataCloudfrontOriginAccessIdentity_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontOriginAccessIdentityData>,
}

#[derive(Clone)]
pub struct DataCloudfrontOriginAccessIdentity(std::rc::Rc<DataCloudfrontOriginAccessIdentity_>);

impl DataCloudfrontOriginAccessIdentity {
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

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_access_identity_path` after provisioning.\n"]
    pub fn cloudfront_access_identity_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_access_identity_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_arn` after provisioning.\n"]
    pub fn iam_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_canonical_user_id` after provisioning.\n"]
    pub fn s3_canonical_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_canonical_user_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfrontOriginAccessIdentity {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontOriginAccessIdentity { }

impl terrars::ToListMappable for DataCloudfrontOriginAccessIdentity {
    type O = terrars::ListRef<DataCloudfrontOriginAccessIdentityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontOriginAccessIdentity_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_origin_access_identity".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontOriginAccessIdentity {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudfrontOriginAccessIdentity {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontOriginAccessIdentity {
        let out = DataCloudfrontOriginAccessIdentity(std::rc::Rc::new(DataCloudfrontOriginAccessIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontOriginAccessIdentityData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontOriginAccessIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginAccessIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontOriginAccessIdentityRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_access_identity_path` after provisioning.\n"]
    pub fn cloudfront_access_identity_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_access_identity_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_arn` after provisioning.\n"]
    pub fn iam_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_canonical_user_id` after provisioning.\n"]
    pub fn s3_canonical_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_canonical_user_id", self.extract_ref()))
    }
}
