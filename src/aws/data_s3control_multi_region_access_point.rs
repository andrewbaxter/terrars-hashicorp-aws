use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3controlMultiRegionAccessPointData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct DataS3controlMultiRegionAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3controlMultiRegionAccessPointData>,
}

#[derive(Clone)]
pub struct DataS3controlMultiRegionAccessPoint(std::rc::Rc<DataS3controlMultiRegionAccessPoint_>);

impl DataS3controlMultiRegionAccessPoint {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(&self) -> terrars::ListRef<DataS3controlMultiRegionAccessPointPublicAccessBlockElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<DataS3controlMultiRegionAccessPointRegionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3controlMultiRegionAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3controlMultiRegionAccessPoint { }

impl terrars::ToListMappable for DataS3controlMultiRegionAccessPoint {
    type O = terrars::ListRef<DataS3controlMultiRegionAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3controlMultiRegionAccessPoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3control_multi_region_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3controlMultiRegionAccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataS3controlMultiRegionAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3controlMultiRegionAccessPoint {
        let out = DataS3controlMultiRegionAccessPoint(std::rc::Rc::new(DataS3controlMultiRegionAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3controlMultiRegionAccessPointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3controlMultiRegionAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3controlMultiRegionAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3controlMultiRegionAccessPointRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(&self) -> terrars::ListRef<DataS3controlMultiRegionAccessPointPublicAccessBlockElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<DataS3controlMultiRegionAccessPointRegionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrict_public_buckets: std::option::Option<terrars::PrimField<bool>>,
}

impl DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    #[doc = "Set the field `block_public_acls`.\n"]
    pub fn set_block_public_acls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_public_acls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_public_policy`.\n"]
    pub fn set_block_public_policy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.block_public_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_public_acls`.\n"]
    pub fn set_ignore_public_acls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_public_acls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restrict_public_buckets`.\n"]
    pub fn set_restrict_public_buckets(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.restrict_public_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    type O = terrars::BlockAssignable<DataS3controlMultiRegionAccessPointPublicAccessBlockEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3controlMultiRegionAccessPointPublicAccessBlockEl {}

impl BuildDataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    pub fn build(self) -> DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
        DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

pub struct DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
        DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_acls` after provisioning.\n"]
    pub fn block_public_acls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `block_public_policy` after provisioning.\n"]
    pub fn block_public_policy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_public_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_public_acls` after provisioning.\n"]
    pub fn ignore_public_acls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `restrict_public_buckets` after provisioning.\n"]
    pub fn restrict_public_buckets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrict_public_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3controlMultiRegionAccessPointRegionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3controlMultiRegionAccessPointRegionsEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3controlMultiRegionAccessPointRegionsEl {
    type O = terrars::BlockAssignable<DataS3controlMultiRegionAccessPointRegionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3controlMultiRegionAccessPointRegionsEl {}

impl BuildDataS3controlMultiRegionAccessPointRegionsEl {
    pub fn build(self) -> DataS3controlMultiRegionAccessPointRegionsEl {
        DataS3controlMultiRegionAccessPointRegionsEl {
            bucket: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataS3controlMultiRegionAccessPointRegionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3controlMultiRegionAccessPointRegionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3controlMultiRegionAccessPointRegionsElRef {
        DataS3controlMultiRegionAccessPointRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3controlMultiRegionAccessPointRegionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
