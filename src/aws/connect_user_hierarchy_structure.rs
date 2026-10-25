#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserHierarchyStructureData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_structure: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure>>,
    dynamic: ConnectUserHierarchyStructureDynamic,
}

struct ConnectUserHierarchyStructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectUserHierarchyStructureData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hierarchy_structure`.\n"]
    pub fn set_hierarchy_structure(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructureRef> {
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
            data: std::cell::RefCell::new(ConnectUserHierarchyStructureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ConnectUserHierarchyStructureDynamic {
    hierarchy_structure: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure>>,
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelFive { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelFive>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelFive {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelFive { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef {
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
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelFour { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelFour>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelFour {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelFour { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef {
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
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelOne { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelOne>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelOne {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelOne { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef {
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
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelThree { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelThree>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelThree {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelThree { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef {
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
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    name: terrars::PrimField<std::string::String>,
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo { }

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo { name: self.name }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
        ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef {
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

#[derive(Serialize, Default)]
struct ConnectUserHierarchyStructure_HierarchyStructureDynamic {
    level_five: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure_LevelFive>>,
    level_four: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure_LevelFour>>,
    level_one: std::option::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure_LevelOne>>,
    level_three: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure_LevelThree>>,
    level_two: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>>,
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructure_HierarchyStructure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure_LevelFive>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure_LevelFour>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure_LevelOne>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure_LevelThree>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<std::vec::Vec<ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>>,
    dynamic: ConnectUserHierarchyStructure_HierarchyStructureDynamic,
}

impl ConnectUserHierarchyStructure_HierarchyStructure {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelFive>>,
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
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelFour>>,
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
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelOne>>,
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
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelThree>>,
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
                    ::Into<terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure_LevelTwo>>,
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

impl terrars::ToListMappable for ConnectUserHierarchyStructure_HierarchyStructure {
    type O = terrars::BlockAssignable<ConnectUserHierarchyStructure_HierarchyStructure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructure_HierarchyStructure {}

impl BuildConnectUserHierarchyStructure_HierarchyStructure {
    pub fn build(self) -> ConnectUserHierarchyStructure_HierarchyStructure {
        ConnectUserHierarchyStructure_HierarchyStructure {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyStructure_HierarchyStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyStructure_HierarchyStructureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyStructure_HierarchyStructureRef {
        ConnectUserHierarchyStructure_HierarchyStructureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructure_HierarchyStructureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructure_LevelFiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructure_LevelFourRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructure_LevelOneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructure_LevelThreeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<ConnectUserHierarchyStructure_HierarchyStructure_LevelTwoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
