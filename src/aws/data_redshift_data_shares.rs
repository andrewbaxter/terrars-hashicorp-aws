#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRedshiftDataSharesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRedshiftDataShares_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRedshiftDataSharesData_>,
}

#[derive(Clone)]
pub struct DataRedshiftDataShares(std::rc::Rc<DataRedshiftDataShares_>);

impl DataRedshiftDataShares {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `data_shares` after provisioning.\n"]
    pub fn data_shares(&self) -> terrars::ListRef<DataRedshiftDataShares_DataSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataRedshiftDataShares {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRedshiftDataShares { }

impl terrars::ToListMappable for DataRedshiftDataShares {
    type O = terrars::ListRef<DataRedshiftDataSharesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRedshiftDataShares_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_redshift_data_shares".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedshiftDataShares {
    pub tf_id: std::string::String,
}

impl BuildDataRedshiftDataShares {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRedshiftDataShares {
        let out = DataRedshiftDataShares(std::rc::Rc::new(DataRedshiftDataShares_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRedshiftDataSharesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedshiftDataSharesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftDataSharesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRedshiftDataSharesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `data_shares` after provisioning.\n"]
    pub fn data_shares(&self) -> terrars::ListRef<DataRedshiftDataShares_DataSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_shares", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRedshiftDataShares_DataShares {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_share_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    producer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedshiftDataShares_DataShares {
    #[doc = "Set the field `data_share_arn`.\n"]
    pub fn set_data_share_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_share_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_by`.\n"]
    pub fn set_managed_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.managed_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `producer_arn`.\n"]
    pub fn set_producer_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.producer_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedshiftDataShares_DataShares {
    type O = terrars::BlockAssignable<DataRedshiftDataShares_DataShares>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftDataShares_DataShares {}

impl BuildDataRedshiftDataShares_DataShares {
    pub fn build(self) -> DataRedshiftDataShares_DataShares {
        DataRedshiftDataShares_DataShares {
            data_share_arn: core::default::Default::default(),
            managed_by: core::default::Default::default(),
            producer_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedshiftDataShares_DataSharesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftDataShares_DataSharesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedshiftDataShares_DataSharesRef {
        DataRedshiftDataShares_DataSharesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftDataShares_DataSharesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_share_arn` after provisioning.\n"]
    pub fn data_share_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_share_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.base))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producer_arn", self.base))
    }
}
