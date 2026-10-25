#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreMemoryData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    event_expiry_duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_key: std::option::Option<std::vec::Vec<BedrockagentcoreMemory_IndexedKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_delivery_resources: std::option::Option<std::vec::Vec<BedrockagentcoreMemory_StreamDeliveryResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreMemory_Timeouts>,
    dynamic: BedrockagentcoreMemoryDynamic,
}

struct BedrockagentcoreMemory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreMemoryData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreMemory(std::rc::Rc<BedrockagentcoreMemory_>);

impl BedrockagentcoreMemory {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_execution_role_arn`.\n"]
    pub fn set_memory_execution_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().memory_execution_role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `indexed_key`.\n"]
    pub fn set_indexed_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreMemory_IndexedKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().indexed_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.indexed_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stream_delivery_resources`.\n"]
    pub fn set_stream_delivery_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stream_delivery_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stream_delivery_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreMemory_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_expiry_duration` after provisioning.\n"]
    pub fn event_expiry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_expiry_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stream_delivery_resources` after provisioning.\n"]
    pub fn stream_delivery_resources(&self) -> terrars::ListRef<BedrockagentcoreMemory_StreamDeliveryResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_delivery_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemory_TimeoutsRef {
        BedrockagentcoreMemory_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreMemory {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreMemory { }

impl terrars::ToListMappable for BedrockagentcoreMemory {
    type O = terrars::ListRef<BedrockagentcoreMemoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreMemory_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_memory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreMemory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub event_expiry_duration: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemory {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreMemory {
        let out = BedrockagentcoreMemory(std::rc::Rc::new(BedrockagentcoreMemory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreMemoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                encryption_key_arn: core::default::Default::default(),
                event_expiry_duration: self.event_expiry_duration,
                memory_execution_role_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                indexed_key: core::default::Default::default(),
                stream_delivery_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreMemoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreMemoryRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_expiry_duration` after provisioning.\n"]
    pub fn event_expiry_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_expiry_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_execution_role_arn` after provisioning.\n"]
    pub fn memory_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `stream_delivery_resources` after provisioning.\n"]
    pub fn stream_delivery_resources(&self) -> terrars::ListRef<BedrockagentcoreMemory_StreamDeliveryResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_delivery_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreMemory_TimeoutsRef {
        BedrockagentcoreMemory_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemoryDynamic {
    indexed_key: std::option::Option<terrars::DynamicBlock<BedrockagentcoreMemory_IndexedKey>>,
    stream_delivery_resources: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemory_StreamDeliveryResources>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_IndexedKey {
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreMemory_IndexedKey { }

impl terrars::ToListMappable for BedrockagentcoreMemory_IndexedKey {
    type O = terrars::BlockAssignable<BedrockagentcoreMemory_IndexedKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_IndexedKey {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemory_IndexedKey {
    pub fn build(self) -> BedrockagentcoreMemory_IndexedKey {
        BedrockagentcoreMemory_IndexedKey {
            key: self.key,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_IndexedKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_IndexedKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreMemory_IndexedKeyRef {
        BedrockagentcoreMemory_IndexedKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_IndexedKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
    pub fn build(self) -> BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
        BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration {
            level: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef {
        BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisDynamic {
    content_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
    data_stream_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration>>,
    dynamic: BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisDynamic,
}

impl BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
    #[doc = "Set the field `content_configuration`.\n"]
    pub fn set_content_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
    type O = terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
    #[doc = ""]
    pub data_stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
    pub fn build(self) -> BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
        BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis {
            data_stream_arn: self.data_stream_arn,
            content_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef {
        BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_stream_arn` after provisioning.\n"]
    pub fn data_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `content_configuration` after provisioning.\n"]
    pub fn content_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis_ContentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemory_StreamDeliveryResources_ResourceDynamic {
    kinesis: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis: std::option::Option<std::vec::Vec<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis>>,
    dynamic: BedrockagentcoreMemory_StreamDeliveryResources_ResourceDynamic,
}

impl BedrockagentcoreMemory_StreamDeliveryResources_Resource {
    #[doc = "Set the field `kinesis`.\n"]
    pub fn set_kinesis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources_Resource_Kinesis>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemory_StreamDeliveryResources_Resource {
    type O = terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource {}

impl BuildBedrockagentcoreMemory_StreamDeliveryResources_Resource {
    pub fn build(self) -> BedrockagentcoreMemory_StreamDeliveryResources_Resource {
        BedrockagentcoreMemory_StreamDeliveryResources_Resource {
            kinesis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef {
        BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis` after provisioning.\n"]
    pub fn kinesis(&self) -> terrars::ListRef<BedrockagentcoreMemory_StreamDeliveryResources_Resource_KinesisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreMemory_StreamDeliveryResourcesDynamic {
    resource: std::option::Option<terrars::DynamicBlock<BedrockagentcoreMemory_StreamDeliveryResources_Resource>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_StreamDeliveryResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<BedrockagentcoreMemory_StreamDeliveryResources_Resource>>,
    dynamic: BedrockagentcoreMemory_StreamDeliveryResourcesDynamic,
}

impl BedrockagentcoreMemory_StreamDeliveryResources {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources_Resource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemory_StreamDeliveryResources {
    type O = terrars::BlockAssignable<BedrockagentcoreMemory_StreamDeliveryResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_StreamDeliveryResources {}

impl BuildBedrockagentcoreMemory_StreamDeliveryResources {
    pub fn build(self) -> BedrockagentcoreMemory_StreamDeliveryResources {
        BedrockagentcoreMemory_StreamDeliveryResources {
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_StreamDeliveryResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_StreamDeliveryResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreMemory_StreamDeliveryResourcesRef {
        BedrockagentcoreMemory_StreamDeliveryResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_StreamDeliveryResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<BedrockagentcoreMemory_StreamDeliveryResources_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreMemory_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreMemory_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreMemory_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreMemory_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreMemory_Timeouts {}

impl BuildBedrockagentcoreMemory_Timeouts {
    pub fn build(self) -> BedrockagentcoreMemory_Timeouts {
        BedrockagentcoreMemory_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreMemory_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreMemory_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreMemory_TimeoutsRef {
        BedrockagentcoreMemory_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreMemory_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
