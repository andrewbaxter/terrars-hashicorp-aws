#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotIndexingConfigurationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_group_indexing_configuration: std
    ::option
    ::Option<std::vec::Vec<IotIndexingConfiguration_ThingGroupIndexingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_indexing_configuration: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration>>,
    dynamic: IotIndexingConfigurationDynamic,
}

struct IotIndexingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IotIndexingConfigurationData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thing_group_indexing_configuration`.\n"]
    pub fn set_thing_group_indexing_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IotIndexingConfiguration_ThingGroupIndexingConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfiguration_ThingGroupIndexingConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfiguration_ThingIndexingConfigurationRef> {
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
            data: std::cell::RefCell::new(IotIndexingConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfiguration_ThingGroupIndexingConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> terrars::ListRef<IotIndexingConfiguration_ThingIndexingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thing_indexing_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationDynamic {
    thing_group_indexing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingGroupIndexingConfiguration>>,
    thing_indexing_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
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

impl terrars::ToListMappable for IotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
    type O = terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfiguration_ThingIndexingConfiguration_CustomField {}

impl BuildIotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
    pub fn build(self) -> IotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
        IotIndexingConfiguration_ThingIndexingConfiguration_CustomField {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_CustomFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfiguration_ThingIndexingConfiguration_CustomFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfiguration_ThingIndexingConfiguration_CustomFieldRef {
        IotIndexingConfiguration_ThingIndexingConfiguration_CustomFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_CustomFieldRef {
    #[allow(dead_code)]
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
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
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

impl terrars::ToListMappable for IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
    type O = terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {}

impl BuildIotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
    pub fn build(self) -> IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
        IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_ManagedFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfiguration_ThingIndexingConfiguration_ManagedFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfiguration_ThingIndexingConfiguration_ManagedFieldRef {
        IotIndexingConfiguration_ThingIndexingConfiguration_ManagedFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_ManagedFieldRef {
    #[allow(dead_code)]
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
struct IotIndexingConfiguration_ThingGroupIndexingConfigurationDynamic {
    custom_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
    managed_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfiguration_ThingGroupIndexingConfiguration {
    thing_group_indexing_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_field: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_field: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>>,
    dynamic: IotIndexingConfiguration_ThingGroupIndexingConfigurationDynamic,
}

impl IotIndexingConfiguration_ThingGroupIndexingConfiguration {
    #[doc = "Set the field `custom_field`.\n"]
    pub fn set_custom_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
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
                        terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>,
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

impl terrars::ToListMappable for IotIndexingConfiguration_ThingGroupIndexingConfiguration {
    type O = terrars::BlockAssignable<IotIndexingConfiguration_ThingGroupIndexingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfiguration_ThingGroupIndexingConfiguration {
    #[doc = ""]
    pub thing_group_indexing_mode: terrars::PrimField<std::string::String>,
}

impl BuildIotIndexingConfiguration_ThingGroupIndexingConfiguration {
    pub fn build(self) -> IotIndexingConfiguration_ThingGroupIndexingConfiguration {
        IotIndexingConfiguration_ThingGroupIndexingConfiguration {
            thing_group_indexing_mode: self.thing_group_indexing_mode,
            custom_field: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotIndexingConfiguration_ThingGroupIndexingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfiguration_ThingGroupIndexingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfiguration_ThingGroupIndexingConfigurationRef {
        IotIndexingConfiguration_ThingGroupIndexingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfiguration_ThingGroupIndexingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_mode` after provisioning.\n"]
    pub fn thing_group_indexing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thing_group_indexing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_shadow_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_Filter {
    #[doc = "Set the field `named_shadow_names`.\n"]
    pub fn set_named_shadow_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.named_shadow_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotIndexingConfiguration_ThingIndexingConfiguration_Filter {
    type O = terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfiguration_ThingIndexingConfiguration_Filter {}

impl BuildIotIndexingConfiguration_ThingIndexingConfiguration_Filter {
    pub fn build(self) -> IotIndexingConfiguration_ThingIndexingConfiguration_Filter {
        IotIndexingConfiguration_ThingIndexingConfiguration_Filter {
            named_shadow_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef {
        IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `named_shadow_names` after provisioning.\n"]
    pub fn named_shadow_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.named_shadow_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfiguration_ThingIndexingConfigurationDynamic {
    custom_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
    filter: std::option::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration_Filter>>,
    managed_field: std
    ::option
    ::Option<terrars::DynamicBlock<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfiguration_ThingIndexingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_defender_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_shadow_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thing_connectivity_indexing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    thing_indexing_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_field: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_field: std::option::Option<std::vec::Vec<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>>,
    dynamic: IotIndexingConfiguration_ThingIndexingConfigurationDynamic,
}

impl IotIndexingConfiguration_ThingIndexingConfiguration {
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
                    ::Into<terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_CustomField>>,
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
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
                        terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration_ManagedField>,
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

impl terrars::ToListMappable for IotIndexingConfiguration_ThingIndexingConfiguration {
    type O = terrars::BlockAssignable<IotIndexingConfiguration_ThingIndexingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfiguration_ThingIndexingConfiguration {
    #[doc = ""]
    pub thing_indexing_mode: terrars::PrimField<std::string::String>,
}

impl BuildIotIndexingConfiguration_ThingIndexingConfiguration {
    pub fn build(self) -> IotIndexingConfiguration_ThingIndexingConfiguration {
        IotIndexingConfiguration_ThingIndexingConfiguration {
            device_defender_indexing_mode: core::default::Default::default(),
            named_shadow_indexing_mode: core::default::Default::default(),
            thing_connectivity_indexing_mode: core::default::Default::default(),
            thing_indexing_mode: self.thing_indexing_mode,
            custom_field: core::default::Default::default(),
            filter: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotIndexingConfiguration_ThingIndexingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotIndexingConfiguration_ThingIndexingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IotIndexingConfiguration_ThingIndexingConfigurationRef {
        IotIndexingConfiguration_ThingIndexingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfiguration_ThingIndexingConfigurationRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<IotIndexingConfiguration_ThingIndexingConfiguration_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
