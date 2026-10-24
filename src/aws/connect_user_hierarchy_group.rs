use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserHierarchyGroupData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct ConnectUserHierarchyGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectUserHierarchyGroupData>,
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
    pub fn hierarchy_path(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElRef> {
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
            data: std::cell::RefCell::new(ConnectUserHierarchyGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                parent_group_id: core::default::Default::default(),
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
    pub fn hierarchy_path(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElRef> {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelFourEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelOneEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
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

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_five: std::option::Option<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_four: std::option::Option<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_one: std::option::Option<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_three: std::option::Option<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level_two: std::option::Option<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
}

impl ConnectUserHierarchyGroupHierarchyPathEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    ) -> Self {
        self.level_five = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    ) -> Self {
        self.level_four = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    ) -> Self {
        self.level_one = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    ) -> Self {
        self.level_three = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
    ) -> Self {
        self.level_two = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConnectUserHierarchyGroupHierarchyPathEl {
    type O = terrars::BlockAssignable<ConnectUserHierarchyGroupHierarchyPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathEl {
        ConnectUserHierarchyGroupHierarchyPathEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectUserHierarchyGroupHierarchyPathElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConnectUserHierarchyGroupHierarchyPathElRef {
        ConnectUserHierarchyGroupHierarchyPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> terrars::ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
