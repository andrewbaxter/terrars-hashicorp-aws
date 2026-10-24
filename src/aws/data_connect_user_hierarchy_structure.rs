use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectUserHierarchyStructureData {
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
}

struct DataConnectUserHierarchyStructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectUserHierarchyStructureData>,
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

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElRef> {
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
            data: std::cell::RefCell::new(DataConnectUserHierarchyStructureData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
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
    pub fn hierarchy_structure(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElRef> {
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
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std
    ::option
    ::Option<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
}

impl DataConnectUserHierarchyStructureHierarchyStructureEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
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
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
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
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
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
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
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
                    ::Into<terrars::ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    ) -> Self {
        self.level_two = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureEl {}

impl BuildDataConnectUserHierarchyStructureHierarchyStructureEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureEl {
        DataConnectUserHierarchyStructureHierarchyStructureEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyStructureHierarchyStructureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyStructureHierarchyStructureElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyStructureHierarchyStructureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
