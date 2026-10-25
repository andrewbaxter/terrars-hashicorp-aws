#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3AccessPointData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataS3AccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3AccessPointData_>,
}

#[derive(Clone)]
pub struct DataS3AccessPoint(std::rc::Rc<DataS3AccessPoint_>);

impl DataS3AccessPoint {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(
        &self,
    ) -> terrars::ListRef<DataS3AccessPoint_PublicAccessBlockConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<DataS3AccessPoint_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3AccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3AccessPoint { }

impl terrars::ToListMappable for DataS3AccessPoint {
    type O = terrars::ListRef<DataS3AccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3AccessPoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3AccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataS3AccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3AccessPoint {
        let out = DataS3AccessPoint(std::rc::Rc::new(DataS3AccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3AccessPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3AccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3AccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3AccessPointRef {
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

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(
        &self,
    ) -> terrars::ListRef<DataS3AccessPoint_PublicAccessBlockConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<DataS3AccessPoint_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataS3AccessPoint_PublicAccessBlockConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_public_policy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_public_acls: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrict_public_buckets: std::option::Option<terrars::PrimField<bool>>,
}

impl DataS3AccessPoint_PublicAccessBlockConfiguration {
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

impl terrars::ToListMappable for DataS3AccessPoint_PublicAccessBlockConfiguration {
    type O = terrars::BlockAssignable<DataS3AccessPoint_PublicAccessBlockConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3AccessPoint_PublicAccessBlockConfiguration {}

impl BuildDataS3AccessPoint_PublicAccessBlockConfiguration {
    pub fn build(self) -> DataS3AccessPoint_PublicAccessBlockConfiguration {
        DataS3AccessPoint_PublicAccessBlockConfiguration {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3AccessPoint_PublicAccessBlockConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3AccessPoint_PublicAccessBlockConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3AccessPoint_PublicAccessBlockConfigurationRef {
        DataS3AccessPoint_PublicAccessBlockConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3AccessPoint_PublicAccessBlockConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataS3AccessPoint_VpcConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3AccessPoint_VpcConfiguration {
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3AccessPoint_VpcConfiguration {
    type O = terrars::BlockAssignable<DataS3AccessPoint_VpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3AccessPoint_VpcConfiguration {}

impl BuildDataS3AccessPoint_VpcConfiguration {
    pub fn build(self) -> DataS3AccessPoint_VpcConfiguration {
        DataS3AccessPoint_VpcConfiguration { vpc_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataS3AccessPoint_VpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3AccessPoint_VpcConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3AccessPoint_VpcConfigurationRef {
        DataS3AccessPoint_VpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3AccessPoint_VpcConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
