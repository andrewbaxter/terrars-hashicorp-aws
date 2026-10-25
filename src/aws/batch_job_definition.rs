#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BatchJobDefinitionData_ {
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
    container_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deregister_on_new_revision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_capabilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_properties: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std::option::Option<std::vec::Vec<BatchJobDefinition_RetryStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<BatchJobDefinition_Timeout>>,
    dynamic: BatchJobDefinitionDynamic,
}

struct BatchJobDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BatchJobDefinitionData_>,
}

#[derive(Clone)]
pub struct BatchJobDefinition(std::rc::Rc<BatchJobDefinition_>);

impl BatchJobDefinition {
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

    #[doc = "Set the field `container_properties`.\n"]
    pub fn set_container_properties(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().container_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deregister_on_new_revision`.\n"]
    pub fn set_deregister_on_new_revision(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deregister_on_new_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecs_properties`.\n"]
    pub fn set_ecs_properties(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ecs_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_properties`.\n"]
    pub fn set_node_properties(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_capabilities`.\n"]
    pub fn set_platform_capabilities(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().platform_capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_priority`.\n"]
    pub fn set_scheduling_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scheduling_priority = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `eks_properties`.\n"]
    pub fn set_eks_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eks_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eks_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_RetryStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_Timeout>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregister_on_new_revision` after provisioning.\n"]
    pub fn deregister_on_new_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deregister_on_new_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_properties` after provisioning.\n"]
    pub fn ecs_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> terrars::ListRef<BatchJobDefinition_EksPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<BatchJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<BatchJobDefinition_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

impl terrars::Referable for BatchJobDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BatchJobDefinition { }

impl terrars::ToListMappable for BatchJobDefinition {
    type O = terrars::ListRef<BatchJobDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BatchJobDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_batch_job_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchJobDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> BatchJobDefinition {
        let out = BatchJobDefinition(std::rc::Rc::new(BatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BatchJobDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_properties: core::default::Default::default(),
                deregister_on_new_revision: core::default::Default::default(),
                ecs_properties: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                node_properties: core::default::Default::default(),
                parameters: core::default::Default::default(),
                platform_capabilities: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                region: core::default::Default::default(),
                scheduling_priority: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                eks_properties: core::default::Default::default(),
                retry_strategy: core::default::Default::default(),
                timeout: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BatchJobDefinitionRef {
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

    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_properties` after provisioning.\n"]
    pub fn container_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deregister_on_new_revision` after provisioning.\n"]
    pub fn deregister_on_new_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deregister_on_new_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_properties` after provisioning.\n"]
    pub fn ecs_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_capabilities` after provisioning.\n"]
    pub fn platform_capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.platform_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> terrars::ListRef<BatchJobDefinition_EksPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<BatchJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<BatchJobDefinition_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinitionDynamic {
    eks_properties: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties>>,
    retry_strategy: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_RetryStrategy>>,
    timeout: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_Timeout>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_Env { }

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Containers_Env {
        BatchJobDefinition_EksProperties_PodProperties_Containers_Env {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
        BatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
        BatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
        BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_privilege_escalation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only_root_file_system: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_as_group: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_as_non_root: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_as_user: std::option::Option<terrars::PrimField<f64>>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    #[doc = "Set the field `allow_privilege_escalation`.\n"]
    pub fn set_allow_privilege_escalation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_privilege_escalation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.privileged = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only_root_file_system`.\n"]
    pub fn set_read_only_root_file_system(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only_root_file_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_group`.\n"]
    pub fn set_run_as_group(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_as_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_non_root`.\n"]
    pub fn set_run_as_non_root(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_as_non_root = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_user`.\n"]
    pub fn set_run_as_user(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_as_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
        BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
            allow_privilege_escalation: core::default::Default::default(),
            privileged: core::default::Default::default(),
            read_only_root_file_system: core::default::Default::default(),
            run_as_group: core::default::Default::default(),
            run_as_non_root: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
        BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_privilege_escalation` after provisioning.\n"]
    pub fn allow_privilege_escalation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_privilege_escalation", self.base))
    }

    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only_root_file_system` after provisioning.\n"]
    pub fn read_only_root_file_system(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only_root_file_system", self.base))
    }

    #[doc = "Get a reference to the value of field `run_as_group` after provisioning.\n"]
    pub fn run_as_group(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_as_group", self.base))
    }

    #[doc = "Get a reference to the value of field `run_as_non_root` after provisioning.\n"]
    pub fn run_as_non_root(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_as_non_root", self.base))
    }

    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\n"]
    pub fn run_as_user(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    mount_path: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    #[doc = ""]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
        BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
            mount_path: self.mount_path,
            name: self.name,
            read_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
        BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_EksProperties_PodProperties_ContainersDynamic {
    env: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    security_context: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    volume_mounts: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_context: std
    ::option
    ::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std
    ::option
    ::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
    dynamic: BatchJobDefinition_EksProperties_PodProperties_ContainersDynamic,
}

impl BatchJobDefinition_EksProperties_PodProperties_Containers {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_pull_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Containers {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Containers {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Containers {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Containers {
        BatchJobDefinition_EksProperties_PodProperties_Containers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            env: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_ContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_ContainersRef {
        BatchJobDefinition_EksProperties_PodProperties_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_pull_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_context", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret {
    name: terrars::PrimField<std::string::String>,
}

impl BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret { }

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_ImagePullSecret {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_ImagePullSecret {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret {
        BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret { name: self.name }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef {
        BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_EksProperties_PodProperties_InitContainersDynamic {
    env: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    security_context: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    volume_mounts: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_InitContainers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_context: std
    ::option
    ::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std
    ::option
    ::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
    dynamic: BatchJobDefinition_EksProperties_PodProperties_InitContainersDynamic,
}

impl BatchJobDefinition_EksProperties_PodProperties_InitContainers {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_pull_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_Resources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_InitContainers {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_InitContainers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_InitContainers {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_InitContainers {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_InitContainers {
        BatchJobDefinition_EksProperties_PodProperties_InitContainers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            env: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
        BatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_pull_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_context", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Metadata {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Metadata {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Metadata {}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Metadata {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Metadata {
        BatchJobDefinition_EksProperties_PodProperties_Metadata { labels: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_MetadataRef {
        BatchJobDefinition_EksProperties_PodProperties_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
    size_limit: terrars::PrimField<std::string::String>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    #[doc = ""]
    pub size_limit: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
            medium: core::default::Default::default(),
            size_limit: self.size_limit,
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }

    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\n"]
    pub fn size_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    path: terrars::PrimField<std::string::String>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath { }

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath { path: self.path }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional: std::option::Option<terrars::PrimField<bool>>,
    secret_name: terrars::PrimField<std::string::String>,
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    #[doc = "Set the field `optional`.\n"]
    pub fn set_optional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.optional = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
            optional: core::default::Default::default(),
            secret_name: self.secret_name,
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
        BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `optional` after provisioning.\n"]
    pub fn optional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optional", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_EksProperties_PodProperties_VolumesDynamic {
    empty_dir: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>>,
    host_path: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>>,
    secret: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_dir: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>>,
    dynamic: BatchJobDefinition_EksProperties_PodProperties_VolumesDynamic,
}

impl BatchJobDefinition_EksProperties_PodProperties_Volumes {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.empty_dir = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties_Volumes {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties_Volumes {}

impl BuildBatchJobDefinition_EksProperties_PodProperties_Volumes {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties_Volumes {
        BatchJobDefinition_EksProperties_PodProperties_Volumes {
            name: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            host_path: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodProperties_VolumesRef {
        BatchJobDefinition_EksProperties_PodProperties_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_EksProperties_PodPropertiesDynamic {
    containers: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Containers>>,
    image_pull_secret: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret>>,
    init_containers: std
    ::option
    ::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_InitContainers>>,
    metadata: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Metadata>>,
    volumes: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties_Volumes>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties_PodProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_process_namespace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Containers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_secret: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_containers: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_InitContainers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties_Volumes>>,
    dynamic: BatchJobDefinition_EksProperties_PodPropertiesDynamic,
}

impl BatchJobDefinition_EksProperties_PodProperties {
    #[doc = "Set the field `dns_policy`.\n"]
    pub fn set_dns_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_network`.\n"]
    pub fn set_host_network(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.host_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_name`.\n"]
    pub fn set_service_account_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_process_namespace`.\n"]
    pub fn set_share_process_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.share_process_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Containers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_pull_secret`.\n"]
    pub fn set_image_pull_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_ImagePullSecret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_pull_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_pull_secret = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `init_containers`.\n"]
    pub fn set_init_containers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_InitContainers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.init_containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.init_containers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties_Volumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties_PodProperties {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties_PodProperties {}

impl BuildBatchJobDefinition_EksProperties_PodProperties {
    pub fn build(self) -> BatchJobDefinition_EksProperties_PodProperties {
        BatchJobDefinition_EksProperties_PodProperties {
            dns_policy: core::default::Default::default(),
            host_network: core::default::Default::default(),
            service_account_name: core::default::Default::default(),
            share_process_namespace: core::default::Default::default(),
            containers: core::default::Default::default(),
            image_pull_secret: core::default::Default::default(),
            init_containers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksProperties_PodPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksProperties_PodPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_EksProperties_PodPropertiesRef {
        BatchJobDefinition_EksProperties_PodPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksProperties_PodPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_policy` after provisioning.\n"]
    pub fn dns_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `host_network` after provisioning.\n"]
    pub fn host_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_network", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\n"]
    pub fn service_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `share_process_namespace` after provisioning.\n"]
    pub fn share_process_namespace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_process_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_secret` after provisioning.\n"]
    pub fn image_pull_secret(
        &self,
    ) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_ImagePullSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_pull_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `init_containers` after provisioning.\n"]
    pub fn init_containers(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_InitContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.init_containers", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodProperties_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_EksPropertiesDynamic {
    pod_properties: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_EksProperties_PodProperties>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_EksProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_properties: std::option::Option<std::vec::Vec<BatchJobDefinition_EksProperties_PodProperties>>,
    dynamic: BatchJobDefinition_EksPropertiesDynamic,
}

impl BatchJobDefinition_EksProperties {
    #[doc = "Set the field `pod_properties`.\n"]
    pub fn set_pod_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_EksProperties_PodProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_EksProperties {
    type O = terrars::BlockAssignable<BatchJobDefinition_EksProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_EksProperties {}

impl BuildBatchJobDefinition_EksProperties {
    pub fn build(self) -> BatchJobDefinition_EksProperties {
        BatchJobDefinition_EksProperties {
            pod_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_EksPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_EksPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchJobDefinition_EksPropertiesRef {
        BatchJobDefinition_EksPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_EksPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pod_properties` after provisioning.\n"]
    pub fn pod_properties(&self) -> terrars::ListRef<BatchJobDefinition_EksProperties_PodPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_RetryStrategy_EvaluateOnExit {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_exit_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BatchJobDefinition_RetryStrategy_EvaluateOnExit {
    #[doc = "Set the field `on_exit_code`.\n"]
    pub fn set_on_exit_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_exit_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_reason`.\n"]
    pub fn set_on_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_status_reason`.\n"]
    pub fn set_on_status_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_status_reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_RetryStrategy_EvaluateOnExit {
    type O = terrars::BlockAssignable<BatchJobDefinition_RetryStrategy_EvaluateOnExit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    pub fn build(self) -> BatchJobDefinition_RetryStrategy_EvaluateOnExit {
        BatchJobDefinition_RetryStrategy_EvaluateOnExit {
            action: self.action,
            on_exit_code: core::default::Default::default(),
            on_reason: core::default::Default::default(),
            on_status_reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
        BatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `on_exit_code` after provisioning.\n"]
    pub fn on_exit_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_exit_code", self.base))
    }

    #[doc = "Get a reference to the value of field `on_reason` after provisioning.\n"]
    pub fn on_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `on_status_reason` after provisioning.\n"]
    pub fn on_status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_status_reason", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobDefinition_RetryStrategyDynamic {
    evaluate_on_exit: std::option::Option<terrars::DynamicBlock<BatchJobDefinition_RetryStrategy_EvaluateOnExit>>,
}

#[derive(Serialize)]
pub struct BatchJobDefinition_RetryStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_on_exit: std::option::Option<std::vec::Vec<BatchJobDefinition_RetryStrategy_EvaluateOnExit>>,
    dynamic: BatchJobDefinition_RetryStrategyDynamic,
}

impl BatchJobDefinition_RetryStrategy {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchJobDefinition_RetryStrategy_EvaluateOnExit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate_on_exit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate_on_exit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_RetryStrategy {
    type O = terrars::BlockAssignable<BatchJobDefinition_RetryStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_RetryStrategy {}

impl BuildBatchJobDefinition_RetryStrategy {
    pub fn build(self) -> BatchJobDefinition_RetryStrategy {
        BatchJobDefinition_RetryStrategy {
            attempts: core::default::Default::default(),
            evaluate_on_exit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_RetryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_RetryStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchJobDefinition_RetryStrategyRef {
        BatchJobDefinition_RetryStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_RetryStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(&self) -> terrars::ListRef<BatchJobDefinition_RetryStrategy_EvaluateOnExitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate_on_exit", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobDefinition_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempt_duration_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BatchJobDefinition_Timeout {
    #[doc = "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchJobDefinition_Timeout {
    type O = terrars::BlockAssignable<BatchJobDefinition_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobDefinition_Timeout {}

impl BuildBatchJobDefinition_Timeout {
    pub fn build(self) -> BatchJobDefinition_Timeout {
        BatchJobDefinition_Timeout { attempt_duration_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BatchJobDefinition_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchJobDefinition_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchJobDefinition_TimeoutRef {
        BatchJobDefinition_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobDefinition_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_duration_seconds", self.base))
    }
}
