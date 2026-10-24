use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectUserHierarchyGroupData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataConnectUserHierarchyGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectUserHierarchyGroupData>,
}

#[derive(Clone)]
pub struct DataConnectUserHierarchyGroup(std::rc::Rc<DataConnectUserHierarchyGroup_>);

impl DataConnectUserHierarchyGroup {
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

    #[doc = "Set the field `hierarchy_group_id`.\n"]
    pub fn set_hierarchy_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hierarchy_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectUserHierarchyGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectUserHierarchyGroup { }

impl terrars::ToListMappable for DataConnectUserHierarchyGroup {
    type O = terrars::ListRef<DataConnectUserHierarchyGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectUserHierarchyGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_user_hierarchy_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectUserHierarchyGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectUserHierarchyGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectUserHierarchyGroup {
        let out = DataConnectUserHierarchyGroup(std::rc::Rc::new(DataConnectUserHierarchyGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectUserHierarchyGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                hierarchy_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectUserHierarchyGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectUserHierarchyGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
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

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    ) -> Self {
        self.level_five = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    ) -> Self {
        self.level_four = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    ) -> Self {
        self.level_one = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    ) -> Self {
        self.level_three = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
    ) -> Self {
        self.level_two = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectUserHierarchyGroupHierarchyPathEl {
    type O = terrars::BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathEl {
        DataConnectUserHierarchyGroupHierarchyPathEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectUserHierarchyGroupHierarchyPathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectUserHierarchyGroupHierarchyPathElRef {
        DataConnectUserHierarchyGroupHierarchyPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
