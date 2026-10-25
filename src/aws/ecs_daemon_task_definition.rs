#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsDaemonTaskDefinitionData_ {
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
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    family: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_definition: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_Volume>>,
    dynamic: EcsDaemonTaskDefinitionDynamic,
}

struct EcsDaemonTaskDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsDaemonTaskDefinitionData_>,
}

#[derive(Clone)]
pub struct EcsDaemonTaskDefinition(std::rc::Rc<EcsDaemonTaskDefinition_>);

impl EcsDaemonTaskDefinition {
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

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().memory = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().task_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_definition`.\n"]
    pub fn set_container_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume`.\n"]
    pub fn set_volume(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_Volume>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volume = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definition` after provisioning.\n"]
    pub fn container_definition(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_definition", self.extract_ref()))
    }
}

impl terrars::Referable for EcsDaemonTaskDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsDaemonTaskDefinition { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition {
    type O = terrars::ListRef<EcsDaemonTaskDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsDaemonTaskDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_daemon_task_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsDaemonTaskDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub family: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsDaemonTaskDefinition {
        let out = EcsDaemonTaskDefinition(std::rc::Rc::new(EcsDaemonTaskDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsDaemonTaskDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cpu: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                family: self.family,
                memory: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                task_role_arn: core::default::Default::default(),
                container_definition: core::default::Default::default(),
                volume: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsDaemonTaskDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsDaemonTaskDefinitionRef {
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

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definition` after provisioning.\n"]
    pub fn container_definition(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_definition", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsDaemonTaskDefinitionDynamic {
    container_definition: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition>>,
    volume: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_Volume>>,
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
    condition: terrars::PrimField<std::string::String>,
    container_name: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_DependsOn { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_DependsOn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
    #[doc = ""]
    pub condition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
        EcsDaemonTaskDefinition_ContainerDefinition_DependsOn {
            condition: self.condition,
            container_name: self.container_name,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef {
        EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_Environment {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_Environment {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_Environment {}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_Environment {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_Environment {
        EcsDaemonTaskDefinition_ContainerDefinition_Environment {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentRef {
        EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentRef {
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
pub struct EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
        EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef {
        EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
        EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration {
            options: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef {
        EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
    command: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retries`.\n"]
    pub fn set_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_period`.\n"]
    pub fn set_start_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
    #[doc = ""]
    pub command: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
        EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck {
            command: self.command,
            interval: core::default::Default::default(),
            retries: core::default::Default::default(),
            start_period: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef {
        EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `retries` after provisioning.\n"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc = "Get a reference to the value of field `start_period` after provisioning.\n"]
    pub fn start_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
    #[doc = "Set the field `add`.\n"]
    pub fn set_add(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drop`.\n"]
    pub fn set_drop(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.drop = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities {
            add: core::default::Default::default(),
            drop: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add` after provisioning.\n"]
    pub fn add(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add", self.base))
    }

    #[doc = "Get a reference to the value of field `drop` after provisioning.\n"]
    pub fn drop(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drop", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    host_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
    #[doc = ""]
    pub host_path: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device {
            container_path: core::default::Default::default(),
            host_path: self.host_path,
            permissions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_path` after provisioning.\n"]
    pub fn container_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
    container_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    size: terrars::PrimField<f64>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mount_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
    #[doc = ""]
    pub container_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs {
            container_path: self.container_path,
            mount_options: core::default::Default::default(),
            size: self.size,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_path` after provisioning.\n"]
    pub fn container_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_options", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersDynamic {
    capabilities: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities>>,
    device: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device>>,
    tmpfs: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs>>,
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_process_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std
    ::option
    ::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs>>,
    dynamic: EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersDynamic,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
    #[doc = "Set the field `init_process_enabled`.\n"]
    pub fn set_init_process_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.init_process_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Capabilities>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capabilities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capabilities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device`.\n"]
    pub fn set_device(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Device>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tmpfs`.\n"]
    pub fn set_tmpfs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_Tmpfs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tmpfs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tmpfs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters {
            init_process_enabled: core::default::Default::default(),
            capabilities: core::default::Default::default(),
            device: core::default::Default::default(),
            tmpfs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `init_process_enabled` after provisioning.\n"]
    pub fn init_process_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init_process_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(
        &self,
    ) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_CapabilitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc = "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_DeviceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs` after provisioning.\n"]
    pub fn tmpfs(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters_TmpfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
    name: terrars::PrimField<std::string::String>,
    value_from: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_from: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
        EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption {
            name: self.name,
            value_from: self.value_from,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationDynamic {
    secret_option: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption>>,
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
    log_driver: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_option: std
    ::option
    ::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption>>,
    dynamic: EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationDynamic,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_option`.\n"]
    pub fn set_secret_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_option = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
    #[doc = ""]
    pub log_driver: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
        EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration {
            log_driver: self.log_driver,
            options: core::default::Default::default(),
            secret_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef {
        EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_option` after provisioning.\n"]
    pub fn secret_option(
        &self,
    ) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration_SecretOptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_option", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_volume: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_volume`.\n"]
    pub fn set_source_volume(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_volume = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_MountPoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_MountPoint {}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
        EcsDaemonTaskDefinition_ContainerDefinition_MountPoint {
            container_path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_volume: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef {
        EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_path` after provisioning.\n"]
    pub fn container_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc = "Get a reference to the value of field `source_volume` after provisioning.\n"]
    pub fn source_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_volume", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
    credentials_parameter: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
    #[doc = ""]
    pub credentials_parameter: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
        EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials {
            credentials_parameter: self.credentials_parameter,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef {
        EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_parameter` after provisioning.\n"]
    pub fn credentials_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignored_exit_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_attempt_period: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
    #[doc = "Set the field `ignored_exit_codes`.\n"]
    pub fn set_ignored_exit_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.ignored_exit_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_attempt_period`.\n"]
    pub fn set_restart_attempt_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.restart_attempt_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
        EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy {
            enabled: self.enabled,
            ignored_exit_codes: core::default::Default::default(),
            restart_attempt_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef {
        EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ignored_exit_codes` after provisioning.\n"]
    pub fn ignored_exit_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignored_exit_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_attempt_period` after provisioning.\n"]
    pub fn restart_attempt_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_attempt_period", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_Secret {
    name: terrars::PrimField<std::string::String>,
    value_from: terrars::PrimField<std::string::String>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_Secret { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_Secret {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_Secret {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_from: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_Secret {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_Secret {
        EcsDaemonTaskDefinition_ContainerDefinition_Secret {
            name: self.name,
            value_from: self.value_from,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_SecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_SecretRef {
        EcsDaemonTaskDefinition_ContainerDefinition_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_SystemControl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_SystemControl {}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
        EcsDaemonTaskDefinition_ContainerDefinition_SystemControl {
            namespace: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef {
        EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
    hard_limit: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    soft_limit: terrars::PrimField<f64>,
}

impl EcsDaemonTaskDefinition_ContainerDefinition_Ulimit { }

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Ulimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
    #[doc = ""]
    pub hard_limit: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub soft_limit: terrars::PrimField<f64>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
        EcsDaemonTaskDefinition_ContainerDefinition_Ulimit {
            hard_limit: self.hard_limit,
            name: self.name,
            soft_limit: self.soft_limit,
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef {
        EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hard_limit` after provisioning.\n"]
    pub fn hard_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hard_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `soft_limit` after provisioning.\n"]
    pub fn soft_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.soft_limit", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsDaemonTaskDefinition_ContainerDefinitionDynamic {
    depends_on: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_DependsOn>>,
    environment: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_Environment>>,
    environment_file: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile>>,
    firelens_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration>>,
    health_check: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck>>,
    linux_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters>>,
    log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration>>,
    mount_point: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_MountPoint>>,
    repository_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials>>,
    restart_policy: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy>>,
    secret: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_Secret>>,
    system_control: std
    ::option
    ::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_SystemControl>>,
    ulimit: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_ContainerDefinition_Ulimit>>,
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_ContainerDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    essential: std::option::Option<terrars::PrimField<bool>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interactive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_reservation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pseudo_terminal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readonly_root_filesystem: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depends_on: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_DependsOn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_file: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firelens_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_parameters: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_point: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_MountPoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_credentials: std
    ::option
    ::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_policy: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_Secret>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_control: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_SystemControl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ulimit: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_ContainerDefinition_Ulimit>>,
    dynamic: EcsDaemonTaskDefinition_ContainerDefinitionDynamic,
}

impl EcsDaemonTaskDefinition_ContainerDefinition {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `essential`.\n"]
    pub fn set_essential(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.essential = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interactive`.\n"]
    pub fn set_interactive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.interactive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_reservation`.\n"]
    pub fn set_memory_reservation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.privileged = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pseudo_terminal`.\n"]
    pub fn set_pseudo_terminal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pseudo_terminal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `readonly_root_filesystem`.\n"]
    pub fn set_readonly_root_filesystem(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.readonly_root_filesystem = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_timeout`.\n"]
    pub fn set_start_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_timeout`.\n"]
    pub fn set_stop_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.stop_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `depends_on`.\n"]
    pub fn set_depends_on(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_DependsOn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.depends_on = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.depends_on = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Environment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_file`.\n"]
    pub fn set_environment_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment_file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firelens_configuration`.\n"]
    pub fn set_firelens_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firelens_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firelens_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_HealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linux_parameters`.\n"]
    pub fn set_linux_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linux_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mount_point`.\n"]
    pub fn set_mount_point(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_MountPoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mount_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mount_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository_credentials`.\n"]
    pub fn set_repository_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restart_policy`.\n"]
    pub fn set_restart_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restart_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restart_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Secret>>,
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

    #[doc = "Set the field `system_control`.\n"]
    pub fn set_system_control(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_SystemControl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.system_control = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.system_control = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ulimit`.\n"]
    pub fn set_ulimit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition_Ulimit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ulimit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ulimit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_ContainerDefinition {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_ContainerDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_ContainerDefinition {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_ContainerDefinition {
    pub fn build(self) -> EcsDaemonTaskDefinition_ContainerDefinition {
        EcsDaemonTaskDefinition_ContainerDefinition {
            command: core::default::Default::default(),
            cpu: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            essential: core::default::Default::default(),
            image: self.image,
            interactive: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_reservation: core::default::Default::default(),
            name: core::default::Default::default(),
            privileged: core::default::Default::default(),
            pseudo_terminal: core::default::Default::default(),
            readonly_root_filesystem: core::default::Default::default(),
            start_timeout: core::default::Default::default(),
            stop_timeout: core::default::Default::default(),
            user: core::default::Default::default(),
            working_directory: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            environment: core::default::Default::default(),
            environment_file: core::default::Default::default(),
            firelens_configuration: core::default::Default::default(),
            health_check: core::default::Default::default(),
            linux_parameters: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            mount_point: core::default::Default::default(),
            repository_credentials: core::default::Default::default(),
            restart_policy: core::default::Default::default(),
            secret: core::default::Default::default(),
            system_control: core::default::Default::default(),
            ulimit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_ContainerDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_ContainerDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsDaemonTaskDefinition_ContainerDefinitionRef {
        EcsDaemonTaskDefinition_ContainerDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_ContainerDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `essential` after provisioning.\n"]
    pub fn essential(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.essential", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `interactive` after provisioning.\n"]
    pub fn interactive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interactive", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }

    #[doc = "Get a reference to the value of field `pseudo_terminal` after provisioning.\n"]
    pub fn pseudo_terminal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pseudo_terminal", self.base))
    }

    #[doc = "Get a reference to the value of field `readonly_root_filesystem` after provisioning.\n"]
    pub fn readonly_root_filesystem(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readonly_root_filesystem", self.base))
    }

    #[doc = "Get a reference to the value of field `start_timeout` after provisioning.\n"]
    pub fn start_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_timeout` after provisioning.\n"]
    pub fn stop_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\n"]
    pub fn depends_on(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_DependsOnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_file` after provisioning.\n"]
    pub fn environment_file(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_EnvironmentFileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_file", self.base))
    }

    #[doc = "Get a reference to the value of field `firelens_configuration` after provisioning.\n"]
    pub fn firelens_configuration(
        &self,
    ) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_FirelensConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firelens_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_parameters` after provisioning.\n"]
    pub fn linux_parameters(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LinuxParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_MountPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_credentials` after provisioning.\n"]
    pub fn repository_credentials(
        &self,
    ) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_RepositoryCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_policy` after provisioning.\n"]
    pub fn restart_policy(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_RestartPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restart_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `system_control` after provisioning.\n"]
    pub fn system_control(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_SystemControlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_control", self.base))
    }

    #[doc = "Get a reference to the value of field `ulimit` after provisioning.\n"]
    pub fn ulimit(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_ContainerDefinition_UlimitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ulimit", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_Volume_Host {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsDaemonTaskDefinition_Volume_Host {
    #[doc = "Set the field `source_path`.\n"]
    pub fn set_source_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_Volume_Host {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_Volume_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_Volume_Host {}

impl BuildEcsDaemonTaskDefinition_Volume_Host {
    pub fn build(self) -> EcsDaemonTaskDefinition_Volume_Host {
        EcsDaemonTaskDefinition_Volume_Host { source_path: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_Volume_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_Volume_HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsDaemonTaskDefinition_Volume_HostRef {
        EcsDaemonTaskDefinition_Volume_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_Volume_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_path` after provisioning.\n"]
    pub fn source_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsDaemonTaskDefinition_VolumeDynamic {
    host: std::option::Option<terrars::DynamicBlock<EcsDaemonTaskDefinition_Volume_Host>>,
}

#[derive(Serialize)]
pub struct EcsDaemonTaskDefinition_Volume {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<EcsDaemonTaskDefinition_Volume_Host>>,
    dynamic: EcsDaemonTaskDefinition_VolumeDynamic,
}

impl EcsDaemonTaskDefinition_Volume {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsDaemonTaskDefinition_Volume_Host>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsDaemonTaskDefinition_Volume {
    type O = terrars::BlockAssignable<EcsDaemonTaskDefinition_Volume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsDaemonTaskDefinition_Volume {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsDaemonTaskDefinition_Volume {
    pub fn build(self) -> EcsDaemonTaskDefinition_Volume {
        EcsDaemonTaskDefinition_Volume {
            name: self.name,
            host: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsDaemonTaskDefinition_VolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsDaemonTaskDefinition_VolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsDaemonTaskDefinition_VolumeRef {
        EcsDaemonTaskDefinition_VolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsDaemonTaskDefinition_VolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::ListRef<EcsDaemonTaskDefinition_Volume_HostRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }
}
