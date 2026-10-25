#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectUserHierarchyStructureData_ {
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
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataConnectUserHierarchyStructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectUserHierarchyStructureData_>,
}

#[derive(Clone)]
pub struct DataConnectUserHierarchyStructure(std::rc::Rc<DataConnectUserHierarchyStructure_>);

impl DataConnectUserHierarchyStructure {
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

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectUserHierarchyStructure {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectUserHierarchyStructure { }

impl terrars::ToListMappable for DataConnectUserHierarchyStructure {
    type O = terrars::ListRef<DataConnectUserHierarchyStructureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectUserHierarchyStructure_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_user_hierarchy_structure".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectUserHierarchyStructure {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectUserHierarchyStructure {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectUserHierarchyStructure {
        let out = DataConnectUserHierarchyStructure(std::rc::Rc::new(DataConnectUserHierarchyStructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectUserHierarchyStructureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectUserHierarchyStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectUserHierarchyStructureRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
        DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>>,
}

impl DataConnectUserHierarchyStructure_HierarchyStructure {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFive>>,
    ) -> Self {
        self.level_five = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFour>>,
    ) -> Self {
        self.level_four = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelOne>>,
    ) -> Self {
        self.level_one = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelThree>>,
    ) -> Self {
        self.level_three = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>>,
    ) -> Self {
        self.level_two = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructure_HierarchyStructure {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructure_HierarchyStructure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructure_HierarchyStructure {}

impl BuildDataConnectUserHierarchyStructure_HierarchyStructure {
    pub fn build(self) -> DataConnectUserHierarchyStructure_HierarchyStructure {
        DataConnectUserHierarchyStructure_HierarchyStructure {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectUserHierarchyStructure_HierarchyStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructure_HierarchyStructureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructure_HierarchyStructureRef {
        DataConnectUserHierarchyStructure_HierarchyStructureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructure_HierarchyStructureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<DataConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
