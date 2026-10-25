#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMqBrokerEngineTypesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMqBrokerEngineTypes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMqBrokerEngineTypesData_>,
}

#[derive(Clone)]
pub struct DataMqBrokerEngineTypes(std::rc::Rc<DataMqBrokerEngineTypes_>);

impl DataMqBrokerEngineTypes {
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

    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_type = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `broker_engine_types` after provisioning.\n"]
    pub fn broker_engine_types(&self) -> terrars::ListRef<DataMqBrokerEngineTypes_BrokerEngineTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_engine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
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

impl terrars::Referable for DataMqBrokerEngineTypes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMqBrokerEngineTypes { }

impl terrars::ToListMappable for DataMqBrokerEngineTypes {
    type O = terrars::ListRef<DataMqBrokerEngineTypesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMqBrokerEngineTypes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_mq_broker_engine_types".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBrokerEngineTypes {
    pub tf_id: std::string::String,
}

impl BuildDataMqBrokerEngineTypes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMqBrokerEngineTypes {
        let out = DataMqBrokerEngineTypes(std::rc::Rc::new(DataMqBrokerEngineTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMqBrokerEngineTypesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                engine_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerEngineTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerEngineTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMqBrokerEngineTypesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `broker_engine_types` after provisioning.\n"]
    pub fn broker_engine_types(&self) -> terrars::ListRef<DataMqBrokerEngineTypes_BrokerEngineTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_engine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
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
pub struct DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {
    type O = terrars::BlockAssignable<DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {}

impl BuildDataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {
    pub fn build(self) -> DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions {
        DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef {
        DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerEngineTypes_BrokerEngineTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_versions: std::option::Option<terrars::ListField<DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions>>,
}

impl DataMqBrokerEngineTypes_BrokerEngineTypes {
    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_versions`.\n"]
    pub fn set_engine_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersions>>,
    ) -> Self {
        self.engine_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerEngineTypes_BrokerEngineTypes {
    type O = terrars::BlockAssignable<DataMqBrokerEngineTypes_BrokerEngineTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEngineTypes_BrokerEngineTypes {}

impl BuildDataMqBrokerEngineTypes_BrokerEngineTypes {
    pub fn build(self) -> DataMqBrokerEngineTypes_BrokerEngineTypes {
        DataMqBrokerEngineTypes_BrokerEngineTypes {
            engine_type: core::default::Default::default(),
            engine_versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBrokerEngineTypes_BrokerEngineTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerEngineTypes_BrokerEngineTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerEngineTypes_BrokerEngineTypesRef {
        DataMqBrokerEngineTypes_BrokerEngineTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEngineTypes_BrokerEngineTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_versions` after provisioning.\n"]
    pub fn engine_versions(&self) -> terrars::ListRef<DataMqBrokerEngineTypes_BrokerEngineTypes_EngineVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_versions", self.base))
    }
}
