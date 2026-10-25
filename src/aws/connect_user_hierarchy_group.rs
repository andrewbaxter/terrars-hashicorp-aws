#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserHierarchyGroupData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct ConnectUserHierarchyGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectUserHierarchyGroupData_>,
}

#[derive(Clone)]
pub struct ConnectUserHierarchyGroup(std::rc::Rc<ConnectUserHierarchyGroup_>);

impl ConnectUserHierarchyGroup {
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

    #[doc = "Set the field `parent_group_id`.\n"]
    pub fn set_parent_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent_group_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
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
    pub fn hierarchy_path(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPathRef> {
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

    #[doc = "Get a reference to the value of field `parent_group_id` after provisioning.\n"]
    pub fn parent_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_group_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectUserHierarchyGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectUserHierarchyGroup { }

impl terrars::ToListMappable for ConnectUserHierarchyGroup {
    type O = terrars::ListRef<ConnectUserHierarchyGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectUserHierarchyGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_user_hierarchy_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUserHierarchyGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConnectUserHierarchyGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectUserHierarchyGroup {
        let out = ConnectUserHierarchyGroup(std::rc::Rc::new(ConnectUserHierarchyGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectUserHierarchyGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                parent_group_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserHierarchyGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectUserHierarchyGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn hierarchy_path(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPathRef> {
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

    #[doc = "Get a reference to the value of field `parent_group_id` after provisioning.\n"]
    pub fn parent_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_group_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelFive {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelFive {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath_LevelFive {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath_LevelFive>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath_LevelFive {}

impl BuildConnectUserHierarchyGroup_HierarchyPath_LevelFive {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath_LevelFive {
        ConnectUserHierarchyGroup_HierarchyPath_LevelFive {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef {
        ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef {
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
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelFour {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelFour {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath_LevelFour {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath_LevelFour>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath_LevelFour {}

impl BuildConnectUserHierarchyGroup_HierarchyPath_LevelFour {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath_LevelFour {
        ConnectUserHierarchyGroup_HierarchyPath_LevelFour {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef {
        ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef {
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
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelOne {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelOne {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath_LevelOne {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath_LevelOne>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath_LevelOne {}

impl BuildConnectUserHierarchyGroup_HierarchyPath_LevelOne {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath_LevelOne {
        ConnectUserHierarchyGroup_HierarchyPath_LevelOne {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef {
        ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef {
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
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelThree {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelThree {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath_LevelThree {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath_LevelThree>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath_LevelThree {}

impl BuildConnectUserHierarchyGroup_HierarchyPath_LevelThree {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath_LevelThree {
        ConnectUserHierarchyGroup_HierarchyPath_LevelThree {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef {
        ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef {
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
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath_LevelTwo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath_LevelTwo {}

impl BuildConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
        ConnectUserHierarchyGroup_HierarchyPath_LevelTwo {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef {
        ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef {
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
pub struct ConnectUserHierarchyGroup_HierarchyPath {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelFive>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelFour>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelOne>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelThree>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelTwo>>,
}

impl ConnectUserHierarchyGroup_HierarchyPath {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelFive>>,
    ) -> Self {
        self.level_five = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelFour>>,
    ) -> Self {
        self.level_four = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelOne>>,
    ) -> Self {
        self.level_one = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelThree>>,
    ) -> Self {
        self.level_three = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroup_HierarchyPath_LevelTwo>>,
    ) -> Self {
        self.level_two = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectUserHierarchyGroup_HierarchyPath {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroup_HierarchyPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroup_HierarchyPath {}

impl BuildConnectUserHierarchyGroup_HierarchyPath {
    pub fn build(self) -> ConnectUserHierarchyGroup_HierarchyPath {
        ConnectUserHierarchyGroup_HierarchyPath {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectUserHierarchyGroup_HierarchyPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroup_HierarchyPathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectUserHierarchyGroup_HierarchyPathRef {
        ConnectUserHierarchyGroup_HierarchyPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroup_HierarchyPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPath_LevelFiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPath_LevelFourRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPath_LevelOneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPath_LevelThreeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<ConnectUserHierarchyGroup_HierarchyPath_LevelTwoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
