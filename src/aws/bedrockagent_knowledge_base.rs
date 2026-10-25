#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentKnowledgeBaseData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_configuration: std::option::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentKnowledgeBase_Timeouts>,
    dynamic: BedrockagentKnowledgeBaseDynamic,
}

struct BedrockagentKnowledgeBase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentKnowledgeBaseData_>,
}

#[derive(Clone)]
pub struct BedrockagentKnowledgeBase(std::rc::Rc<BedrockagentKnowledgeBase_>);

impl BedrockagentKnowledgeBase {
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

    #[doc = "Set the field `knowledge_base_configuration`.\n"]
    pub fn set_knowledge_base_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_base_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_base_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentKnowledgeBase_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_configuration` after provisioning.\n"]
    pub fn knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentKnowledgeBase_TimeoutsRef {
        BedrockagentKnowledgeBase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentKnowledgeBase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentKnowledgeBase { }

impl terrars::ToListMappable for BedrockagentKnowledgeBase {
    type O = terrars::ListRef<BedrockagentKnowledgeBaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentKnowledgeBase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagent_knowledge_base".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentKnowledgeBase {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentKnowledgeBase {
        let out = BedrockagentKnowledgeBase(std::rc::Rc::new(BedrockagentKnowledgeBase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentKnowledgeBaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                knowledge_base_configuration: core::default::Default::default(),
                storage_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentKnowledgeBaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentKnowledgeBaseRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_configuration` after provisioning.\n"]
    pub fn knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentKnowledgeBase_TimeoutsRef {
        BedrockagentKnowledgeBase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBaseDynamic {
    knowledge_base_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration>>,
    storage_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
    kendra_index_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration { }

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
    #[doc = ""]
    pub kendra_index_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration {
            kendra_index_arn: self.kendra_index_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kendra_index_arn` after provisioning.\n"]
    pub fn kendra_index_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kendra_index_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
    fixed_length_duration: terrars::PrimField<f64>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {

}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
    #[doc = ""]
    pub fixed_length_duration: terrars::PrimField<f64>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration {
            fixed_length_duration: self.fixed_length_duration,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_length_duration` after provisioning.\n"]
    pub fn fixed_length_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_length_duration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioDynamic {
    segmentation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
    #[doc = "Set the field `segmentation_configuration`.\n"]
    pub fn set_segmentation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.segmentation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.segmentation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio {
            segmentation_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `segmentation_configuration` after provisioning.\n"]
    pub fn segmentation_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segmentation_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoDynamic {
    segmentation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segmentation_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
    #[doc = "Set the field `segmentation_configuration`.\n"]
    pub fn set_segmentation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.segmentation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.segmentation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video {
            segmentation_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `segmentation_configuration` after provisioning.\n"]
    pub fn segmentation_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio_SegmentationConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segmentation_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationDynamic {
    audio: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio,
        >,
    >,
    video: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_data_type`.\n"]
    pub fn set_embedding_data_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio`.\n"]
    pub fn set_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Audio,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video`.\n"]
    pub fn set_video(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_Video,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration {
            dimensions: core::default::Default::default(),
            embedding_data_type: core::default::Default::default(),
            audio: core::default::Default::default(),
            video: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_data_type` after provisioning.\n"]
    pub fn embedding_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_AudioRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio", self.base))
    }

    #[doc = "Get a reference to the value of field `video` after provisioning.\n"]
    pub fn video(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration_VideoRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationDynamic {
    bedrock_embedding_model_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_embedding_model_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
    #[doc = "Set the field `bedrock_embedding_model_configuration`.\n"]
    pub fn set_bedrock_embedding_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_embedding_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_embedding_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration {
            bedrock_embedding_model_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_embedding_model_configuration` after provisioning.\n"]
    pub fn bedrock_embedding_model_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration_BedrockEmbeddingModelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_embedding_model_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationDynamic {
    embedding_model_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
        >,
    >,
    server_side_encryption_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
    #[doc = "Set the field `embedding_model_arn`.\n"]
    pub fn set_embedding_model_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_model_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_type`.\n"]
    pub fn set_embedding_model_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_model_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_configuration`.\n"]
    pub fn set_embedding_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.embedding_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.embedding_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_side_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_side_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration {
            embedding_model_arn: core::default::Default::default(),
            embedding_model_type: core::default::Default::default(),
            embedding_model_configuration: core::default::Default::default(),
            server_side_encryption_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `embedding_model_arn` after provisioning.\n"]
    pub fn embedding_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_type` after provisioning.\n"]
    pub fn embedding_model_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_type", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_configuration` after provisioning.\n"]
    pub fn embedding_model_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.embedding_model_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration_ServerSideEncryptionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password_secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
    #[doc = "Set the field `database_user`.\n"]
    pub fn set_database_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username_password_secret_arn`.\n"]
    pub fn set_username_password_secret_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.username_password_secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration {
            database_user: core::default::Default::default(),
            type_: self.type_,
            username_password_secret_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_user` after provisioning.\n"]
    pub fn database_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_user", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `username_password_secret_arn` after provisioning.\n"]
    pub fn username_password_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_password_secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationDynamic {
    auth_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
    cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
    #[doc = "Set the field `auth_configuration`.\n"]
    pub fn set_auth_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration {
            cluster_identifier: self.cluster_identifier,
            auth_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_configuration` after provisioning.\n"]
    pub fn auth_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration_AuthConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password_secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
    #[doc = "Set the field `username_password_secret_arn`.\n"]
    pub fn set_username_password_secret_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.username_password_secret_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration {
            type_: self.type_,
            username_password_secret_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `username_password_secret_arn` after provisioning.\n"]
    pub fn username_password_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_password_secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationDynamic {
    auth_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
    workgroup_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
    #[doc = "Set the field `auth_configuration`.\n"]
    pub fn set_auth_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
    #[doc = ""]
    pub workgroup_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration {
            workgroup_arn: self.workgroup_arn,
            auth_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workgroup_arn` after provisioning.\n"]
    pub fn workgroup_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_configuration` after provisioning.\n"]
    pub fn auth_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration_AuthConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationDynamic {
    provisioned_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration,
        >,
    >,
    serverless_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serverless_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
    #[doc = "Set the field `provisioned_configuration`.\n"]
    pub fn set_provisioned_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioned_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioned_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serverless_configuration`.\n"]
    pub fn set_serverless_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serverless_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serverless_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration {
            type_: self.type_,
            provisioned_configuration: core::default::Default::default(),
            serverless_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_configuration` after provisioning.\n"]
    pub fn provisioned_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ProvisionedConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `serverless_configuration` after provisioning.\n"]
    pub fn serverless_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration_ServerlessConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serverless_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
    natural_language: terrars::PrimField<std::string::String>,
    sql: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {

}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
    #[doc = ""]
    pub natural_language: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery {
            natural_language: self.natural_language,
            sql: self.sql,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `natural_language` after provisioning.\n"]
    pub fn natural_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.natural_language", self.base))
    }

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion`.\n"]
    pub fn set_inclusion(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column {
            description: core::default::Default::default(),
            inclusion: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion` after provisioning.\n"]
    pub fn inclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusion", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableDynamic {
    column: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion`.\n"]
    pub fn set_inclusion(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inclusion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_Column,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table {
            description: core::default::Default::default(),
            inclusion: core::default::Default::default(),
            name: self.name,
            column: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion` after provisioning.\n"]
    pub fn inclusion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusion", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table_ColumnRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextDynamic {
    curated_query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery,
        >,
    >,
    table: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    curated_query: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
    #[doc = "Set the field `curated_query`.\n"]
    pub fn set_curated_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQuery,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.curated_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.curated_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_Table,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext {
            curated_query: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `curated_query` after provisioning.\n"]
    pub fn curated_query(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_CuratedQueryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.curated_query", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext_TableRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationDynamic {
    generation_context: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_context: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
    #[doc = "Set the field `execution_timeout_seconds`.\n"]
    pub fn set_execution_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.execution_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generation_context`.\n"]
    pub fn set_generation_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContext,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_context = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration {
            execution_timeout_seconds: core::default::Default::default(),
            generation_context: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_timeout_seconds` after provisioning.\n"]
    pub fn execution_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_context` after provisioning.\n"]
    pub fn generation_context(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration_GenerationContextRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_context", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
    table_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {

}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
    #[doc = ""]
    pub table_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration {
            table_names: self.table_names,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_names` after provisioning.\n"]
    pub fn table_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.table_names", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
    database_name: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {

}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration {
            database_name: self.database_name,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationDynamic {
    aws_data_catalog_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration,
        >,
    >,
    redshift_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_data_catalog_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
    #[doc = "Set the field `aws_data_catalog_configuration`.\n"]
    pub fn set_aws_data_catalog_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_data_catalog_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_data_catalog_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_configuration`.\n"]
    pub fn set_redshift_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration {
            type_: self.type_,
            aws_data_catalog_configuration: core::default::Default::default(),
            redshift_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_data_catalog_configuration` after provisioning.\n"]
    pub fn aws_data_catalog_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_AwsDataCatalogConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_data_catalog_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration_RedshiftConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationDynamic {
    query_engine_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration,
        >,
    >,
    query_generation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration,
        >,
    >,
    storage_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_engine_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_generation_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
    #[doc = "Set the field `query_engine_configuration`.\n"]
    pub fn set_query_engine_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_engine_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_engine_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_generation_configuration`.\n"]
    pub fn set_query_generation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_generation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_generation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration {
            query_engine_configuration: core::default::Default::default(),
            query_generation_configuration: core::default::Default::default(),
            storage_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_engine_configuration` after provisioning.\n"]
    pub fn query_engine_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryEngineConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_engine_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `query_generation_configuration` after provisioning.\n"]
    pub fn query_generation_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_QueryGenerationConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_generation_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration_StorageConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationDynamic {
    redshift_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
    #[doc = "Set the field `redshift_configuration`.\n"]
    pub fn set_redshift_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
    type O =
        terrars::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration {
            type_: self.type_,
            redshift_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_configuration` after provisioning.\n"]
    pub fn redshift_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration_RedshiftConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
    uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {

}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location {
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationDynamic {
    s3_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_location: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3Location,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation {
            type_: self.type_,
            s3_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation_S3LocationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationDynamic {
    storage_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_location: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
    #[doc = "Set the field `storage_location`.\n"]
    pub fn set_storage_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration {
            storage_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_location` after provisioning.\n"]
    pub fn storage_location(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration_StorageLocationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationDynamic {
    embedding_model_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
        >,
    >,
    supplemental_data_storage_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
    embedding_model_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supplemental_data_storage_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration,
        >,
    >,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
    #[doc = "Set the field `embedding_model_configuration`.\n"]
    pub fn set_embedding_model_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.embedding_model_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.embedding_model_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `supplemental_data_storage_configuration`.\n"]
    pub fn set_supplemental_data_storage_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.supplemental_data_storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.supplemental_data_storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
    #[doc = ""]
    pub embedding_model_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration {
            embedding_model_arn: self.embedding_model_arn,
            embedding_model_configuration: core::default::Default::default(),
            supplemental_data_storage_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `embedding_model_arn` after provisioning.\n"]
    pub fn embedding_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_configuration` after provisioning.\n"]
    pub fn embedding_model_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_EmbeddingModelConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.embedding_model_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `supplemental_data_storage_configuration` after provisioning.\n"]
    pub fn supplemental_data_storage_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration_SupplementalDataStorageConfigurationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supplemental_data_storage_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_KnowledgeBaseConfigurationDynamic {
    kendra_knowledge_base_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration>,
    >,
    managed_knowledge_base_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration>,
    >,
    sql_knowledge_base_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration>,
    >,
    vector_knowledge_base_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kendra_knowledge_base_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_knowledge_base_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_knowledge_base_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_knowledge_base_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration>>,
    dynamic: BedrockagentKnowledgeBase_KnowledgeBaseConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
    #[doc = "Set the field `kendra_knowledge_base_configuration`.\n"]
    pub fn set_kendra_knowledge_base_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kendra_knowledge_base_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_knowledge_base_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_knowledge_base_configuration`.\n"]
    pub fn set_managed_knowledge_base_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_knowledge_base_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_knowledge_base_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_knowledge_base_configuration`.\n"]
    pub fn set_sql_knowledge_base_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_knowledge_base_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_knowledge_base_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vector_knowledge_base_configuration`.\n"]
    pub fn set_vector_knowledge_base_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vector_knowledge_base_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_knowledge_base_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
        BedrockagentKnowledgeBase_KnowledgeBaseConfiguration {
            type_: self.type_,
            kendra_knowledge_base_configuration: core::default::Default::default(),
            managed_knowledge_base_configuration: core::default::Default::default(),
            sql_knowledge_base_configuration: core::default::Default::default(),
            vector_knowledge_base_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef {
        BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_KnowledgeBaseConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_knowledge_base_configuration` after provisioning.\n"]
    pub fn kendra_knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_KendraKnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_knowledge_base_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_knowledge_base_configuration` after provisioning.\n"]
    pub fn managed_knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_ManagedKnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_knowledge_base_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_knowledge_base_configuration` after provisioning.\n"]
    pub fn sql_knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_SqlKnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_knowledge_base_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_knowledge_base_configuration` after provisioning.\n"]
    pub fn vector_knowledge_base_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_KnowledgeBaseConfiguration_VectorKnowledgeBaseConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_knowledge_base_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
    metadata_field: terrars::PrimField<std::string::String>,
    text_field: terrars::PrimField<std::string::String>,
    vector_field: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping { }

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
    type O =
        terrars
        ::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
    #[doc = ""]
    pub metadata_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_field: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
        BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping {
            metadata_field: self.metadata_field,
            text_field: self.text_field,
            vector_field: self.vector_field,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef {
        BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
    collection_name: terrars::PrimField<std::string::String>,
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    vector_index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
    #[doc = "Set the field `endpoint_service_name`.\n"]
    pub fn set_endpoint_service_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_index_name`.\n"]
    pub fn set_text_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
    #[doc = ""]
    pub collection_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_index_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration {
            collection_name: self.collection_name,
            credentials_secret_arn: self.credentials_secret_arn,
            database_name: self.database_name,
            endpoint: self.endpoint,
            endpoint_service_name: core::default::Default::default(),
            text_index_name: core::default::Default::default(),
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_name` after provisioning.\n"]
    pub fn collection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_service_name` after provisioning.\n"]
    pub fn endpoint_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `text_index_name` after provisioning.\n"]
    pub fn text_index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
    metadata_field: terrars::PrimField<std::string::String>,
    text_field: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping { }

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
    type O =
        terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
    #[doc = ""]
    pub metadata_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_field: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
        BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping {
            metadata_field: self.metadata_field,
            text_field: self.text_field,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef {
        BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
    graph_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
    #[doc = ""]
    pub graph_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration {
            graph_arn: self.graph_arn,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `graph_arn` after provisioning.\n"]
    pub fn graph_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
    domain_arn: terrars::PrimField<std::string::String>,
    domain_endpoint: terrars::PrimField<std::string::String>,
    vector_index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
    #[doc = ""]
    pub domain_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_index_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration {
            domain_arn: self.domain_arn,
            domain_endpoint: self.domain_endpoint,
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_arn` after provisioning.\n"]
    pub fn domain_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint` after provisioning.\n"]
    pub fn domain_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
    collection_arn: terrars::PrimField<std::string::String>,
    vector_index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
    type O =
        terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
    #[doc = ""]
    pub collection_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_index_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration {
            collection_arn: self.collection_arn,
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_arn` after provisioning.\n"]
    pub fn collection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
    connection_string: terrars::PrimField<std::string::String>,
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
    #[doc = ""]
    pub connection_string: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration {
            connection_string: self.connection_string,
            credentials_secret_arn: self.credentials_secret_arn,
            namespace: core::default::Default::default(),
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_string` after provisioning.\n"]
    pub fn connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_string", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_metadata_field: std::option::Option<terrars::PrimField<std::string::String>>,
    metadata_field: terrars::PrimField<std::string::String>,
    primary_key_field: terrars::PrimField<std::string::String>,
    text_field: terrars::PrimField<std::string::String>,
    vector_field: terrars::PrimField<std::string::String>,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
    #[doc = "Set the field `custom_metadata_field`.\n"]
    pub fn set_custom_metadata_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_metadata_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
    #[doc = ""]
    pub metadata_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub primary_key_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_field: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
        BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping {
            custom_metadata_field: core::default::Default::default(),
            metadata_field: self.metadata_field,
            primary_key_field: self.primary_key_field,
            text_field: self.text_field,
            vector_field: self.vector_field,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef {
        BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_metadata_field` after provisioning.\n"]
    pub fn custom_metadata_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key_field` after provisioning.\n"]
    pub fn primary_key_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping>>,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    resource_arn: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration {
            credentials_secret_arn: self.credentials_secret_arn,
            database_name: self.database_name,
            resource_arn: self.resource_arn,
            table_name: self.table_name,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
    #[doc = "Set the field `metadata_field`.\n"]
    pub fn set_metadata_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_field`.\n"]
    pub fn set_text_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_field`.\n"]
    pub fn set_vector_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vector_field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
    pub fn build(
        self,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
        BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping {
            metadata_field: core::default::Default::default(),
            text_field: core::default::Default::default(),
            vector_field: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef {
        BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_field` after provisioning.\n"]
    pub fn metadata_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_field", self.base))
    }

    #[doc = "Get a reference to the value of field `text_field` after provisioning.\n"]
    pub fn text_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_field", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_field` after provisioning.\n"]
    pub fn vector_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationDynamic {
    field_mapping: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
    credentials_secret_arn: terrars::PrimField<std::string::String>,
    endpoint: terrars::PrimField<std::string::String>,
    vector_index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_mapping: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping>,
    >,
    dynamic: BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
    #[doc = "Set the field `field_mapping`.\n"]
    pub fn set_field_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
    type O =
        terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
    #[doc = ""]
    pub credentials_secret_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vector_index_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration {
            credentials_secret_arn: self.credentials_secret_arn,
            endpoint: self.endpoint,
            vector_index_name: self.vector_index_name,
            field_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_index_name` after provisioning.\n"]
    pub fn vector_index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_mapping` after provisioning.\n"]
    pub fn field_mapping(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration_FieldMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_bucket_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
    #[doc = "Set the field `index_arn`.\n"]
    pub fn set_index_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\n"]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_bucket_arn`.\n"]
    pub fn set_vector_bucket_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vector_bucket_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration {
            index_arn: core::default::Default::default(),
            index_name: core::default::Default::default(),
            vector_bucket_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `index_arn` after provisioning.\n"]
    pub fn index_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_bucket_arn` after provisioning.\n"]
    pub fn vector_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_bucket_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentKnowledgeBase_StorageConfigurationDynamic {
    mongo_db_atlas_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration>>,
    neptune_analytics_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration>>,
    opensearch_managed_cluster_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration>,
    >,
    opensearch_serverless_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration>>,
    pinecone_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration>>,
    rds_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration>>,
    redis_enterprise_cloud_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration>>,
    s3_vectors_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_StorageConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongo_db_atlas_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    neptune_analytics_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearch_managed_cluster_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opensearch_serverless_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinecone_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_enterprise_cloud_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_vectors_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration>>,
    dynamic: BedrockagentKnowledgeBase_StorageConfigurationDynamic,
}

impl BedrockagentKnowledgeBase_StorageConfiguration {
    #[doc = "Set the field `mongo_db_atlas_configuration`.\n"]
    pub fn set_mongo_db_atlas_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongo_db_atlas_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongo_db_atlas_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `neptune_analytics_configuration`.\n"]
    pub fn set_neptune_analytics_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.neptune_analytics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.neptune_analytics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearch_managed_cluster_configuration`.\n"]
    pub fn set_opensearch_managed_cluster_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.opensearch_managed_cluster_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.opensearch_managed_cluster_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `opensearch_serverless_configuration`.\n"]
    pub fn set_opensearch_serverless_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.opensearch_serverless_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.opensearch_serverless_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pinecone_configuration`.\n"]
    pub fn set_pinecone_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pinecone_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pinecone_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_configuration`.\n"]
    pub fn set_rds_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redis_enterprise_cloud_configuration`.\n"]
    pub fn set_redis_enterprise_cloud_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redis_enterprise_cloud_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redis_enterprise_cloud_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_vectors_configuration`.\n"]
    pub fn set_s3_vectors_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_vectors_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_vectors_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentKnowledgeBase_StorageConfiguration {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_StorageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_StorageConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentKnowledgeBase_StorageConfiguration {
    pub fn build(self) -> BedrockagentKnowledgeBase_StorageConfiguration {
        BedrockagentKnowledgeBase_StorageConfiguration {
            type_: self.type_,
            mongo_db_atlas_configuration: core::default::Default::default(),
            neptune_analytics_configuration: core::default::Default::default(),
            opensearch_managed_cluster_configuration: core::default::Default::default(),
            opensearch_serverless_configuration: core::default::Default::default(),
            pinecone_configuration: core::default::Default::default(),
            rds_configuration: core::default::Default::default(),
            redis_enterprise_cloud_configuration: core::default::Default::default(),
            s3_vectors_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_StorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_StorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentKnowledgeBase_StorageConfigurationRef {
        BedrockagentKnowledgeBase_StorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_StorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `mongo_db_atlas_configuration` after provisioning.\n"]
    pub fn mongo_db_atlas_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_MongoDbAtlasConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongo_db_atlas_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `neptune_analytics_configuration` after provisioning.\n"]
    pub fn neptune_analytics_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_NeptuneAnalyticsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.neptune_analytics_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `opensearch_managed_cluster_configuration` after provisioning.\n"]
    pub fn opensearch_managed_cluster_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchManagedClusterConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.opensearch_managed_cluster_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `opensearch_serverless_configuration` after provisioning.\n"]
    pub fn opensearch_serverless_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_OpensearchServerlessConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opensearch_serverless_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `pinecone_configuration` after provisioning.\n"]
    pub fn pinecone_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_PineconeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pinecone_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_configuration` after provisioning.\n"]
    pub fn rds_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_RdsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_enterprise_cloud_configuration` after provisioning.\n"]
    pub fn redis_enterprise_cloud_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_RedisEnterpriseCloudConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_enterprise_cloud_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_vectors_configuration` after provisioning.\n"]
    pub fn s3_vectors_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentKnowledgeBase_StorageConfiguration_S3VectorsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_vectors_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentKnowledgeBase_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentKnowledgeBase_Timeouts {
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

impl terrars::ToListMappable for BedrockagentKnowledgeBase_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentKnowledgeBase_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentKnowledgeBase_Timeouts {}

impl BuildBedrockagentKnowledgeBase_Timeouts {
    pub fn build(self) -> BedrockagentKnowledgeBase_Timeouts {
        BedrockagentKnowledgeBase_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentKnowledgeBase_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentKnowledgeBase_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentKnowledgeBase_TimeoutsRef {
        BedrockagentKnowledgeBase_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentKnowledgeBase_TimeoutsRef {
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
