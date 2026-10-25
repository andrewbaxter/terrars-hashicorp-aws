#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppflowFlowData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_flow_config: std::option::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_catalog_config: std::option::Option<std::vec::Vec<AppflowFlow_MetadataCatalogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_flow_config: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task: std::option::Option<std::vec::Vec<AppflowFlow_Task>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_config: std::option::Option<std::vec::Vec<AppflowFlow_TriggerConfig>>,
    dynamic: AppflowFlowDynamic,
}

struct AppflowFlow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppflowFlowData_>,
}

#[derive(Clone)]
pub struct AppflowFlow(std::rc::Rc<AppflowFlow_>);

impl AppflowFlow {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_arn`.\n"]
    pub fn set_kms_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination_flow_config`.\n"]
    pub fn set_destination_flow_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_flow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_flow_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_catalog_config`.\n"]
    pub fn set_metadata_catalog_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_MetadataCatalogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_catalog_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_catalog_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_flow_config`.\n"]
    pub fn set_source_flow_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_flow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_flow_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task`.\n"]
    pub fn set_task(self, v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_Task>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_config`.\n"]
    pub fn set_trigger_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_TriggerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_config = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `flow_status` after provisioning.\n"]
    pub fn flow_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_flow_config` after provisioning.\n"]
    pub fn destination_flow_config(&self) -> terrars::ListRef<AppflowFlow_DestinationFlowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_catalog_config` after provisioning.\n"]
    pub fn metadata_catalog_config(&self) -> terrars::ListRef<AppflowFlow_MetadataCatalogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_catalog_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> terrars::ListRef<AppflowFlow_TriggerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppflowFlow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppflowFlow { }

impl terrars::ToListMappable for AppflowFlow {
    type O = terrars::ListRef<AppflowFlowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppflowFlow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appflow_flow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppflowFlow {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow {
    pub fn build(self, stack: &mut terrars::Stack) -> AppflowFlow {
        let out = AppflowFlow(std::rc::Rc::new(AppflowFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppflowFlowData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_flow_config: core::default::Default::default(),
                metadata_catalog_config: core::default::Default::default(),
                source_flow_config: core::default::Default::default(),
                task: core::default::Default::default(),
                trigger_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppflowFlowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppflowFlowRef {
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

    #[doc = "Get a reference to the value of field `flow_status` after provisioning.\n"]
    pub fn flow_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `destination_flow_config` after provisioning.\n"]
    pub fn destination_flow_config(&self) -> terrars::ListRef<AppflowFlow_DestinationFlowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_catalog_config` after provisioning.\n"]
    pub fn metadata_catalog_config(&self) -> terrars::ListRef<AppflowFlow_MetadataCatalogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_catalog_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> terrars::ListRef<AppflowFlow_TriggerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDynamic {
    destination_flow_config: std::option::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig>>,
    metadata_catalog_config: std::option::Option<terrars::DynamicBlock<AppflowFlow_MetadataCatalogConfig>>,
    source_flow_config: std::option::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig>>,
    task: std::option::Option<terrars::DynamicBlock<AppflowFlow_Task>>,
    trigger_config: std::option::Option<terrars::DynamicBlock<AppflowFlow_TriggerConfig>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    entity_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
    type O =
        terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
    #[doc = ""]
    pub entity_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
            id_field_names: core::default::Default::default(),
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_type_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
    #[doc = "Set the field `object_type_name`.\n"]
    pub fn set_object_type_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_type_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
    type O =
        terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles {
            domain_name: self.domain_name,
            object_type_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type_name` after provisioning.\n"]
    pub fn object_type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_type_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics { }

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {
    type O =
        terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics {}
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    intermediate_bucket_name: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
    #[doc = ""]
    pub intermediate_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_file_size: std::option::Option<terrars::PrimField<f64>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_file_size`.\n"]
    pub fn set_target_file_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_file_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig {
            aggregation_type: core::default::Default::default(),
            target_file_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_file_size` after provisioning.\n"]
    pub fn target_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_file_size", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_hierarchy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_hierarchy`.\n"]
    pub fn set_prefix_hierarchy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prefix_hierarchy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_type`.\n"]
    pub fn set_prefix_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig {
            prefix_format: core::default::Default::default(),
            prefix_hierarchy: core::default::Default::default(),
            prefix_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_hierarchy` after provisioning.\n"]
    pub fn prefix_hierarchy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_hierarchy", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigDynamic {
    aggregation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig,
        >,
    >,
    prefix_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_source_data_typing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_source_data_typing`.\n"]
    pub fn set_preserve_source_data_typing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_source_data_typing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
    type O =
        terrars
        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig {
            file_type: core::default::Default::default(),
            preserve_source_data_typing: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_source_data_typing` after provisioning.\n"]
    pub fn preserve_source_data_typing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_source_data_typing", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_AggregationConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig_PrefixConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Dynamic {
    s3_output_format_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_format_config: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig>>,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Dynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3 {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3_S3OutputFormatConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transfer_api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
    #[doc = "Set the field `data_transfer_api`.\n"]
    pub fn set_data_transfer_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_transfer_api = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce {
            data_transfer_api: core::default::Default::default(),
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_api` after provisioning.\n"]
    pub fn data_transfer_api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_api", self.base))
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    success_response_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_response_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_response_handling_config`.\n"]
    pub fn set_success_response_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_response_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_response_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
    #[doc = ""]
    pub object_path: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData {
            id_field_names: core::default::Default::default(),
            object_path: self.object_path,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            success_response_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object_path` after provisioning.\n"]
    pub fn object_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_path", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `success_response_handling_config` after provisioning.\n"]
    pub fn success_response_handling_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData_SuccessResponseHandlingConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_response_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    intermediate_bucket_name: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
    #[doc = ""]
    pub intermediate_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig {
            aggregation_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_hierarchy: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    prefix_type: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_hierarchy`.\n"]
    pub fn set_prefix_hierarchy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prefix_hierarchy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
    #[doc = ""]
    pub prefix_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig {
            prefix_format: core::default::Default::default(),
            prefix_hierarchy: core::default::Default::default(),
            prefix_type: self.prefix_type,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_hierarchy` after provisioning.\n"]
    pub fn prefix_hierarchy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_hierarchy", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigDynamic {
    aggregation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig,
        >,
    >,
    prefix_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig,
        >,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
    pub fn build(
        self,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig {
            file_type: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_AggregationConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig_PrefixConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverDynamic {
    s3_output_format_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_format_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver_S3OutputFormatConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig>,
    >,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk {
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo_ErrorHandlingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesDynamic {
    custom_connector: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector>>,
    customer_profiles: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles>,
    >,
    event_bridge: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge>>,
    honeycode: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode>>,
    lookout_metrics: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics>>,
    marketo: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo>>,
    redshift: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift>>,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3>>,
    salesforce: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce>>,
    sapo_data: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData>>,
    snowflake: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake>>,
    upsolver: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver>>,
    zendesk: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_profiles: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_bridge: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookout_metrics: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std::option::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upsolver: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std::option::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk>>,
    dynamic: AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesDynamic,
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnector,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customer_profiles`.\n"]
    pub fn set_customer_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfiles,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_profiles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_profiles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_bridge`.\n"]
    pub fn set_event_bridge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridge,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_bridge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_bridge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Honeycode>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.honeycode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lookout_metrics`.\n"]
    pub fn set_lookout_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lookout_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lookout_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Marketo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Redshift>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Salesforce>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoData>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Snowflake>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upsolver`.\n"]
    pub fn set_upsolver(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Upsolver>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upsolver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upsolver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_Zendesk>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {}

impl BuildAppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties {
            custom_connector: core::default::Default::default(),
            customer_profiles: core::default::Default::default(),
            event_bridge: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            lookout_metrics: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            upsolver: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef {
        AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_profiles` after provisioning.\n"]
    pub fn customer_profiles(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_CustomerProfilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_profiles", self.base))
    }

    #[doc = "Get a reference to the value of field `event_bridge` after provisioning.\n"]
    pub fn event_bridge(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_EventBridgeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bridge", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_HoneycodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `lookout_metrics` after provisioning.\n"]
    pub fn lookout_metrics(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_LookoutMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lookout_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_MarketoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_RedshiftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SalesforceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SapoDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_SnowflakeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `upsolver` after provisioning.\n"]
    pub fn upsolver(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_UpsolverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upsolver", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties_ZendeskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_DestinationFlowConfigDynamic {
    destination_connector_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_DestinationFlowConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_connector_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties>>,
    dynamic: AppflowFlow_DestinationFlowConfigDynamic,
}

impl AppflowFlow_DestinationFlowConfig {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_connector_properties`.\n"]
    pub fn set_destination_connector_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig_DestinationConnectorProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_connector_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_connector_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_DestinationFlowConfig {
    type O = terrars::BlockAssignable<AppflowFlow_DestinationFlowConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_DestinationFlowConfig {
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_DestinationFlowConfig {
    pub fn build(self) -> AppflowFlow_DestinationFlowConfig {
        AppflowFlow_DestinationFlowConfig {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            destination_connector_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_DestinationFlowConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_DestinationFlowConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_DestinationFlowConfigRef {
        AppflowFlow_DestinationFlowConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_DestinationFlowConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_connector_properties` after provisioning.\n"]
    pub fn destination_connector_properties(
        &self,
    ) -> terrars::ListRef<AppflowFlow_DestinationFlowConfig_DestinationConnectorPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
    database_name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    table_prefix: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_MetadataCatalogConfig_GlueDataCatalog { }

impl terrars::ToListMappable for AppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
    type O = terrars::BlockAssignable<AppflowFlow_MetadataCatalogConfig_GlueDataCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_prefix: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
    pub fn build(self) -> AppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
        AppflowFlow_MetadataCatalogConfig_GlueDataCatalog {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_prefix: self.table_prefix,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef {
        AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_MetadataCatalogConfigDynamic {
    glue_data_catalog: std::option::Option<terrars::DynamicBlock<AppflowFlow_MetadataCatalogConfig_GlueDataCatalog>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_MetadataCatalogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_data_catalog: std::option::Option<std::vec::Vec<AppflowFlow_MetadataCatalogConfig_GlueDataCatalog>>,
    dynamic: AppflowFlow_MetadataCatalogConfigDynamic,
}

impl AppflowFlow_MetadataCatalogConfig {
    #[doc = "Set the field `glue_data_catalog`.\n"]
    pub fn set_glue_data_catalog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_MetadataCatalogConfig_GlueDataCatalog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glue_data_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_data_catalog = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_MetadataCatalogConfig {
    type O = terrars::BlockAssignable<AppflowFlow_MetadataCatalogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_MetadataCatalogConfig {}

impl BuildAppflowFlow_MetadataCatalogConfig {
    pub fn build(self) -> AppflowFlow_MetadataCatalogConfig {
        AppflowFlow_MetadataCatalogConfig {
            glue_data_catalog: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_MetadataCatalogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_MetadataCatalogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_MetadataCatalogConfigRef {
        AppflowFlow_MetadataCatalogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_MetadataCatalogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `glue_data_catalog` after provisioning.\n"]
    pub fn glue_data_catalog(&self) -> terrars::ListRef<AppflowFlow_MetadataCatalogConfig_GlueDataCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glue_data_catalog", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_IncrementalPullConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datetime_type_field_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_SourceFlowConfig_IncrementalPullConfig {
    #[doc = "Set the field `datetime_type_field_name`.\n"]
    pub fn set_datetime_type_field_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.datetime_type_field_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_IncrementalPullConfig {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_IncrementalPullConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_IncrementalPullConfig {}

impl BuildAppflowFlow_SourceFlowConfig_IncrementalPullConfig {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_IncrementalPullConfig {
        AppflowFlow_SourceFlowConfig_IncrementalPullConfig {
            datetime_type_field_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef {
        AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datetime_type_field_name` after provisioning.\n"]
    pub fn datetime_type_field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datetime_type_field_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    entity_name: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
    #[doc = ""]
    pub entity_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
    #[doc = "Set the field `s3_input_file_type`.\n"]
    pub fn set_s3_input_file_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_input_file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig {
            s3_input_file_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_input_file_type` after provisioning.\n"]
    pub fn s3_input_file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_file_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Dynamic {
    s3_input_format_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
    bucket_name: terrars::PrimField<std::string::String>,
    bucket_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_format_config: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig>>,
    dynamic: AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Dynamic,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
    #[doc = "Set the field `s3_input_format_config`.\n"]
    pub fn set_s3_input_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_input_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_input_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bucket_prefix: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3 {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            s3_input_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_format_config` after provisioning.\n"]
    pub fn s3_input_format_config(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3_S3InputFormatConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_input_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transfer_api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dynamic_field_update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_deleted_records: std::option::Option<terrars::PrimField<bool>>,
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
    #[doc = "Set the field `data_transfer_api`.\n"]
    pub fn set_data_transfer_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_transfer_api = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dynamic_field_update`.\n"]
    pub fn set_enable_dynamic_field_update(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_dynamic_field_update = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_deleted_records`.\n"]
    pub fn set_include_deleted_records(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_deleted_records = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce {
            data_transfer_api: core::default::Default::default(),
            enable_dynamic_field_update: core::default::Default::default(),
            include_deleted_records: core::default::Default::default(),
            object: self.object,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_api` after provisioning.\n"]
    pub fn data_transfer_api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_api", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_dynamic_field_update` after provisioning.\n"]
    pub fn enable_dynamic_field_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dynamic_field_update", self.base))
    }

    #[doc = "Get a reference to the value of field `include_deleted_records` after provisioning.\n"]
    pub fn include_deleted_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_deleted_records", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
    max_page_size: terrars::PrimField<f64>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
    type O =
        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
    #[doc = ""]
    pub max_page_size: terrars::PrimField<f64>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig {
            max_page_size: self.max_page_size,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_page_size` after provisioning.\n"]
    pub fn max_page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_page_size", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
    max_page_size: terrars::PrimField<f64>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
    type O =
        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
    #[doc = ""]
    pub max_page_size: terrars::PrimField<f64>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig {
            max_page_size: self.max_page_size,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_page_size` after provisioning.\n"]
    pub fn max_page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_page_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataDynamic {
    pagination_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig>>,
    parallelism_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
    object_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pagination_config: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_config: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig>>,
    dynamic: AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataDynamic,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
    #[doc = "Set the field `pagination_config`.\n"]
    pub fn set_pagination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pagination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pagination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelism_config`.\n"]
    pub fn set_parallelism_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parallelism_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
    #[doc = ""]
    pub object_path: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData {
            object_path: self.object_path,
            pagination_config: core::default::Default::default(),
            parallelism_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_path` after provisioning.\n"]
    pub fn object_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_path", self.base))
    }

    #[doc = "Get a reference to the value of field `pagination_config` after provisioning.\n"]
    pub fn pagination_config(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_PaginationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pagination_config", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_config` after provisioning.\n"]
    pub fn parallelism_config(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData_ParallelismConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_all_versions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_renditions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_source_files: std::option::Option<terrars::PrimField<bool>>,
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
    #[doc = "Set the field `document_type`.\n"]
    pub fn set_document_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_all_versions`.\n"]
    pub fn set_include_all_versions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_all_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_renditions`.\n"]
    pub fn set_include_renditions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_renditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_source_files`.\n"]
    pub fn set_include_source_files(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_source_files = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva {
            document_type: core::default::Default::default(),
            include_all_versions: core::default::Default::default(),
            include_renditions: core::default::Default::default(),
            include_source_files: core::default::Default::default(),
            object: self.object,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.base))
    }

    #[doc = "Get a reference to the value of field `include_all_versions` after provisioning.\n"]
    pub fn include_all_versions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_all_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_renditions` after provisioning.\n"]
    pub fn include_renditions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_renditions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_source_files` after provisioning.\n"]
    pub fn include_source_files(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_source_files", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk { }

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk { object: self.object }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesDynamic {
    amplitude: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude>>,
    custom_connector: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector>>,
    datadog: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog>>,
    dynatrace: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace>>,
    google_analytics: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics>>,
    infor_nexus: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus>>,
    marketo: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo>>,
    s3: std::option::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3>>,
    salesforce: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce>>,
    sapo_data: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData>>,
    service_now: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow>>,
    singular: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular>>,
    slack: std::option::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack>>,
    trendmicro: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro>>,
    veeva: std::option::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva>>,
    zendesk: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk>>,
    dynamic: AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesDynamic,
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorProperties {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Amplitude>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amplitude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnector>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Datadog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Dynatrace>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynatrace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalytics>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexus>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infor_nexus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Marketo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Salesforce>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNow>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Singular>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.singular = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Slack>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Trendmicro>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trendmicro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Veeva>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.veeva = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_Zendesk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig_SourceConnectorProperties {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties {}

impl BuildAppflowFlow_SourceFlowConfig_SourceConnectorProperties {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig_SourceConnectorProperties {
        AppflowFlow_SourceFlowConfig_SourceConnectorProperties {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef {
        AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_AmplitudeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_CustomConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_DynatraceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_GoogleAnalyticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_InforNexusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_MarketoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SalesforceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SapoDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ServiceNowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SingularRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_SlackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_TrendmicroRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_VeevaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorProperties_ZendeskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_SourceFlowConfigDynamic {
    incremental_pull_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_IncrementalPullConfig>>,
    source_connector_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlow_SourceFlowConfig_SourceConnectorProperties>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_SourceFlowConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_pull_config: std::option::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_IncrementalPullConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_connector_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowFlow_SourceFlowConfig_SourceConnectorProperties>>,
    dynamic: AppflowFlow_SourceFlowConfigDynamic,
}

impl AppflowFlow_SourceFlowConfig {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incremental_pull_config`.\n"]
    pub fn set_incremental_pull_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_IncrementalPullConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.incremental_pull_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.incremental_pull_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_connector_properties`.\n"]
    pub fn set_source_connector_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_SourceFlowConfig_SourceConnectorProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_connector_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_connector_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_SourceFlowConfig {
    type O = terrars::BlockAssignable<AppflowFlow_SourceFlowConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_SourceFlowConfig {
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_SourceFlowConfig {
    pub fn build(self) -> AppflowFlow_SourceFlowConfig {
        AppflowFlow_SourceFlowConfig {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            incremental_pull_config: core::default::Default::default(),
            source_connector_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_SourceFlowConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_SourceFlowConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_SourceFlowConfigRef {
        AppflowFlow_SourceFlowConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_SourceFlowConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `incremental_pull_config` after provisioning.\n"]
    pub fn incremental_pull_config(&self) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_IncrementalPullConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incremental_pull_config", self.base))
    }

    #[doc = "Get a reference to the value of field `source_connector_properties` after provisioning.\n"]
    pub fn source_connector_properties(
        &self,
    ) -> terrars::ListRef<AppflowFlow_SourceFlowConfig_SourceConnectorPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_Task_ConnectorOperator {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_Task_ConnectorOperator {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amplitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_connector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dynatrace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.google_analytics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.infor_nexus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.marketo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.salesforce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sapo_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_now = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.singular = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slack = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trendmicro = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.veeva = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zendesk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_Task_ConnectorOperator {
    type O = terrars::BlockAssignable<AppflowFlow_Task_ConnectorOperator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_Task_ConnectorOperator {}

impl BuildAppflowFlow_Task_ConnectorOperator {
    pub fn build(self) -> AppflowFlow_Task_ConnectorOperator {
        AppflowFlow_Task_ConnectorOperator {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_Task_ConnectorOperatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_Task_ConnectorOperatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_Task_ConnectorOperatorRef {
        AppflowFlow_Task_ConnectorOperatorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_Task_ConnectorOperatorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_TaskDynamic {
    connector_operator: std::option::Option<terrars::DynamicBlock<AppflowFlow_Task_ConnectorOperator>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_Task {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    task_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_operator: std::option::Option<std::vec::Vec<AppflowFlow_Task_ConnectorOperator>>,
    dynamic: AppflowFlow_TaskDynamic,
}

impl AppflowFlow_Task {
    #[doc = "Set the field `destination_field`.\n"]
    pub fn set_destination_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_fields`.\n"]
    pub fn set_source_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_properties`.\n"]
    pub fn set_task_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.task_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_operator`.\n"]
    pub fn set_connector_operator(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_Task_ConnectorOperator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector_operator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_operator = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_Task {
    type O = terrars::BlockAssignable<AppflowFlow_Task>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_Task {
    #[doc = ""]
    pub task_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_Task {
    pub fn build(self) -> AppflowFlow_Task {
        AppflowFlow_Task {
            destination_field: core::default::Default::default(),
            source_fields: core::default::Default::default(),
            task_properties: core::default::Default::default(),
            task_type: self.task_type,
            connector_operator: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_TaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_TaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_TaskRef {
        AppflowFlow_TaskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_TaskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_field` after provisioning.\n"]
    pub fn destination_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_field", self.base))
    }

    #[doc = "Get a reference to the value of field `source_fields` after provisioning.\n"]
    pub fn source_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `task_properties` after provisioning.\n"]
    pub fn task_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.task_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_operator` after provisioning.\n"]
    pub fn connector_operator(&self) -> terrars::ListRef<AppflowFlow_Task_ConnectorOperatorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_pull_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_execution_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
    #[doc = "Set the field `data_pull_mode`.\n"]
    pub fn set_data_pull_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_pull_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_execution_from`.\n"]
    pub fn set_first_execution_from(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.first_execution_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_end_time`.\n"]
    pub fn set_schedule_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_offset`.\n"]
    pub fn set_schedule_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.schedule_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_start_time`.\n"]
    pub fn set_schedule_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
    type O = terrars::BlockAssignable<AppflowFlow_TriggerConfig_TriggerProperties_Scheduled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
    pub fn build(self) -> AppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
        AppflowFlow_TriggerConfig_TriggerProperties_Scheduled {
            data_pull_mode: core::default::Default::default(),
            first_execution_from: core::default::Default::default(),
            schedule_end_time: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            schedule_offset: core::default::Default::default(),
            schedule_start_time: core::default::Default::default(),
            timezone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef {
        AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_pull_mode` after provisioning.\n"]
    pub fn data_pull_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_pull_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `first_execution_from` after provisioning.\n"]
    pub fn first_execution_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_execution_from", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_end_time` after provisioning.\n"]
    pub fn schedule_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_offset` after provisioning.\n"]
    pub fn schedule_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_start_time` after provisioning.\n"]
    pub fn schedule_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_TriggerConfig_TriggerPropertiesDynamic {
    scheduled: std::option::Option<terrars::DynamicBlock<AppflowFlow_TriggerConfig_TriggerProperties_Scheduled>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_TriggerConfig_TriggerProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled: std::option::Option<std::vec::Vec<AppflowFlow_TriggerConfig_TriggerProperties_Scheduled>>,
    dynamic: AppflowFlow_TriggerConfig_TriggerPropertiesDynamic,
}

impl AppflowFlow_TriggerConfig_TriggerProperties {
    #[doc = "Set the field `scheduled`.\n"]
    pub fn set_scheduled(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_TriggerConfig_TriggerProperties_Scheduled>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_TriggerConfig_TriggerProperties {
    type O = terrars::BlockAssignable<AppflowFlow_TriggerConfig_TriggerProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_TriggerConfig_TriggerProperties {}

impl BuildAppflowFlow_TriggerConfig_TriggerProperties {
    pub fn build(self) -> AppflowFlow_TriggerConfig_TriggerProperties {
        AppflowFlow_TriggerConfig_TriggerProperties {
            scheduled: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_TriggerConfig_TriggerPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_TriggerConfig_TriggerPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlow_TriggerConfig_TriggerPropertiesRef {
        AppflowFlow_TriggerConfig_TriggerPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_TriggerConfig_TriggerPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled` after provisioning.\n"]
    pub fn scheduled(&self) -> terrars::ListRef<AppflowFlow_TriggerConfig_TriggerProperties_ScheduledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlow_TriggerConfigDynamic {
    trigger_properties: std::option::Option<terrars::DynamicBlock<AppflowFlow_TriggerConfig_TriggerProperties>>,
}

#[derive(Serialize)]
pub struct AppflowFlow_TriggerConfig {
    trigger_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_properties: std::option::Option<std::vec::Vec<AppflowFlow_TriggerConfig_TriggerProperties>>,
    dynamic: AppflowFlow_TriggerConfigDynamic,
}

impl AppflowFlow_TriggerConfig {
    #[doc = "Set the field `trigger_properties`.\n"]
    pub fn set_trigger_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlow_TriggerConfig_TriggerProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlow_TriggerConfig {
    type O = terrars::BlockAssignable<AppflowFlow_TriggerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlow_TriggerConfig {
    #[doc = ""]
    pub trigger_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow_TriggerConfig {
    pub fn build(self) -> AppflowFlow_TriggerConfig {
        AppflowFlow_TriggerConfig {
            trigger_type: self.trigger_type,
            trigger_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowFlow_TriggerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlow_TriggerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlow_TriggerConfigRef {
        AppflowFlow_TriggerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlow_TriggerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trigger_type` after provisioning.\n"]
    pub fn trigger_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_type", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_properties` after provisioning.\n"]
    pub fn trigger_properties(&self) -> terrars::ListRef<AppflowFlow_TriggerConfig_TriggerPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_properties", self.base))
    }
}
