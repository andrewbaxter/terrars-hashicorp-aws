use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotIndexingConfigurationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_group_indexing_configuration: std
    ::option
    ::Option<std::vec::Vec<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_indexing_configuration: std
    ::option
    ::Option<std::vec::Vec<IotIndexingConfigurationThingIndexingConfigurationEl>>,
    dynamic: IotIndexingConfigurationDynamic,
}

struct IotIndexingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IotIndexingConfigurationData>,
}

#[derive(Clone)]
pub struct IotIndexingConfiguration(std::rc::Rc<IotIndexingConfiguration_>);

impl IotIndexingConfiguration {
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

    #[doc = "Set the field `thing_group_indexing_configuration`.\n"]
    pub fn set_thing_group_indexing_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thing_group_indexing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thing_group_indexing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thing_indexing_configuration`.\n"]
    pub fn set_thing_indexing_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thing_indexing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thing_indexing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfigurationThingGroupIndexingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfigurationThingIndexingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thing_indexing_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for IotIndexingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IotIndexingConfiguration { }

impl terrars::ToListMappable for IotIndexingConfiguration {
    type O = terrars::ListRef<IotIndexingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IotIndexingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_iot_indexing_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotIndexingConfiguration {
    pub tf_id: std::string::String,
}

impl BuildIotIndexingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> IotIndexingConfiguration {
        let out = IotIndexingConfiguration(std::rc::Rc::new(IotIndexingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IotIndexingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                thing_group_indexing_configuration: core::default::Default::default(),
                thing_indexing_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotIndexingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IotIndexingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfigurationThingGroupIndexingConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfigurationThingIndexingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thing_indexing_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationThingGroupIndexingConfigurationElDynamic {
    custom_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>>,
    managed_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    thing_group_indexing_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_field: std
    ::option
    ::Option<std::vec::Vec<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_field: std
    ::option
    ::Option<std::vec::Vec<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>>,
    dynamic: IotIndexingConfigurationThingGroupIndexingConfigurationElDynamic,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    #[doc = "Set the field `custom_field`.\n"]
    pub fn set_custom_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_field`.\n"]
    pub fn set_managed_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationEl {
    #[doc = ""]
    pub thing_group_indexing_mode: terrars::PrimField<std::string::String>,
}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationEl {
            thing_group_indexing_mode: self.thing_group_indexing_mode,
            custom_field: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_mode` after provisioning.\n"]
    pub fn thing_group_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thing_group_indexing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {}

impl BuildIotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
        IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
        IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {}

impl BuildIotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
        IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
        IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationThingIndexingConfigurationElDynamic {
    custom_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>>,
    managed_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_defender_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_shadow_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_connectivity_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    thing_indexing_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_field: std::option::Option<std::vec::Vec<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_field: std::option::Option<std::vec::Vec<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>>,
    dynamic: IotIndexingConfigurationThingIndexingConfigurationElDynamic,
}

impl IotIndexingConfigurationThingIndexingConfigurationEl {
    #[doc = "Set the field `device_defender_indexing_mode`.\n"]
    pub fn set_device_defender_indexing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.device_defender_indexing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `named_shadow_indexing_mode`.\n"]
    pub fn set_named_shadow_indexing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.named_shadow_indexing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thing_connectivity_indexing_mode`.\n"]
    pub fn set_thing_connectivity_indexing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.thing_connectivity_indexing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_field`.\n"]
    pub fn set_custom_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_field`.\n"]
    pub fn set_managed_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfigurationThingIndexingConfigurationEl {
    type O = terrars::BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationEl {
    #[doc = ""]
    pub thing_indexing_mode: terrars::PrimField<std::string::String>,
}

impl BuildIotIndexingConfigurationThingIndexingConfigurationEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationEl {
        IotIndexingConfigurationThingIndexingConfigurationEl {
            device_defender_indexing_mode: core::default::Default::default(),
            named_shadow_indexing_mode: core::default::Default::default(),
            thing_connectivity_indexing_mode: core::default::Default::default(),
            thing_indexing_mode: self.thing_indexing_mode,
            custom_field: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfigurationThingIndexingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElRef {
        IotIndexingConfigurationThingIndexingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_defender_indexing_mode` after provisioning.\n"]
    pub fn device_defender_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_defender_indexing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `named_shadow_indexing_mode` after provisioning.\n"]
    pub fn named_shadow_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.named_shadow_indexing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `thing_connectivity_indexing_mode` after provisioning.\n"]
    pub fn thing_connectivity_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thing_connectivity_indexing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `thing_indexing_mode` after provisioning.\n"]
    pub fn thing_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thing_indexing_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationDynamic {
    thing_group_indexing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    thing_indexing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationEl>>,
}
