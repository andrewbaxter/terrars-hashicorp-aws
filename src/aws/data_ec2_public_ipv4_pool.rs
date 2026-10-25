#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2PublicIpv4PoolData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEc2PublicIpv4Pool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2PublicIpv4PoolData_>,
}

#[derive(Clone)]
pub struct DataEc2PublicIpv4Pool(std::rc::Rc<DataEc2PublicIpv4Pool_>);

impl DataEc2PublicIpv4Pool {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_border_group` after provisioning.\n"]
    pub fn network_border_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_border_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_address_ranges` after provisioning.\n"]
    pub fn pool_address_ranges(&self) -> terrars::ListRef<DataEc2PublicIpv4Pool_PoolAddressRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_address_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `total_address_count` after provisioning.\n"]
    pub fn total_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_available_address_count` after provisioning.\n"]
    pub fn total_available_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_available_address_count", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataEc2PublicIpv4Pool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2PublicIpv4Pool { }

impl terrars::ToListMappable for DataEc2PublicIpv4Pool {
    type O = terrars::ListRef<DataEc2PublicIpv4PoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2PublicIpv4Pool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_public_ipv4_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2PublicIpv4Pool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEc2PublicIpv4Pool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2PublicIpv4Pool {
        let out = DataEc2PublicIpv4Pool(std::rc::Rc::new(DataEc2PublicIpv4Pool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2PublicIpv4PoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                pool_id: self.pool_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2PublicIpv4PoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2PublicIpv4PoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2PublicIpv4PoolRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_border_group` after provisioning.\n"]
    pub fn network_border_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_border_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_address_ranges` after provisioning.\n"]
    pub fn pool_address_ranges(&self) -> terrars::ListRef<DataEc2PublicIpv4Pool_PoolAddressRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_address_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\n"]
    pub fn pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `total_address_count` after provisioning.\n"]
    pub fn total_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_available_address_count` after provisioning.\n"]
    pub fn total_available_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_available_address_count", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2PublicIpv4Pool_PoolAddressRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2PublicIpv4Pool_PoolAddressRanges {
    #[doc = "Set the field `address_count`.\n"]
    pub fn set_address_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_address_count`.\n"]
    pub fn set_available_address_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_address`.\n"]
    pub fn set_first_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_address`.\n"]
    pub fn set_last_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2PublicIpv4Pool_PoolAddressRanges {
    type O = terrars::BlockAssignable<DataEc2PublicIpv4Pool_PoolAddressRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2PublicIpv4Pool_PoolAddressRanges {}

impl BuildDataEc2PublicIpv4Pool_PoolAddressRanges {
    pub fn build(self) -> DataEc2PublicIpv4Pool_PoolAddressRanges {
        DataEc2PublicIpv4Pool_PoolAddressRanges {
            address_count: core::default::Default::default(),
            available_address_count: core::default::Default::default(),
            first_address: core::default::Default::default(),
            last_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2PublicIpv4Pool_PoolAddressRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2PublicIpv4Pool_PoolAddressRangesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2PublicIpv4Pool_PoolAddressRangesRef {
        DataEc2PublicIpv4Pool_PoolAddressRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2PublicIpv4Pool_PoolAddressRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_count` after provisioning.\n"]
    pub fn address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_count", self.base))
    }

    #[doc = "Get a reference to the value of field `available_address_count` after provisioning.\n"]
    pub fn available_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_address_count", self.base))
    }

    #[doc = "Get a reference to the value of field `first_address` after provisioning.\n"]
    pub fn first_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_address", self.base))
    }

    #[doc = "Get a reference to the value of field `last_address` after provisioning.\n"]
    pub fn last_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_address", self.base))
    }
}
