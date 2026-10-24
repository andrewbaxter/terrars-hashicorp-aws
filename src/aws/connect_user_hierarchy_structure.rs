use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserHierarchyStructureData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_structure: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureEl>>,
    dynamic: ConnectUserHierarchyStructureDynamic,
}

struct ConnectUserHierarchyStructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectUserHierarchyStructureData>,
}

#[derive(Clone)]
pub struct ConnectUserHierarchyStructure(std::rc::Rc<ConnectUserHierarchyStructure_>);

impl ConnectUserHierarchyStructure {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `hierarchy_structure`.\n"]
    pub fn set_hierarchy_structure(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hierarchy_structure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hierarchy_structure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectUserHierarchyStructure {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectUserHierarchyStructure { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure {
    type O = terrars::ListRef<ConnectUserHierarchyStructureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectUserHierarchyStructure_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_user_hierarchy_structure".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUserHierarchyStructure {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectUserHierarchyStructure {
        let out = ConnectUserHierarchyStructure(std::rc::Rc::new(ConnectUserHierarchyStructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectUserHierarchyStructureData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                hierarchy_structure: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserHierarchyStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectUserHierarchyStructureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl { }

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
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
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl { }

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
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
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl { }

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
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
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl { }

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
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
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl { }

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
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

#[derive(Serialize, Default)]
struct ConnectUserHierarchyStructureHierarchyStructureElDynamic {
    level_five: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    level_four: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    level_one: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    level_three: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    level_two: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    dynamic: ConnectUserHierarchyStructureHierarchyStructureElDynamic,
}

impl ConnectUserHierarchyStructureHierarchyStructureEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_five = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_five = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_four = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_four = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_one = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_one = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_three = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_three = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.level_two = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.level_two = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConnectUserHierarchyStructureHierarchyStructureEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureEl {}

impl BuildConnectUserHierarchyStructureHierarchyStructureEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureEl {
        ConnectUserHierarchyStructureHierarchyStructureEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructureHierarchyStructureElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructureHierarchyStructureElRef {
        ConnectUserHierarchyStructureHierarchyStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectUserHierarchyStructureDynamic {
    hierarchy_structure: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureEl>>,
}
