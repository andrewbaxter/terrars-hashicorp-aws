#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerAppImageConfigData_ {
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
    app_image_config_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_editor_app_image_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_CodeEditorAppImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_lab_image_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_image_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_KernelGatewayImageConfig>>,
    dynamic: SagemakerAppImageConfigDynamic,
}

struct SagemakerAppImageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerAppImageConfigData_>,
}

#[derive(Clone)]
pub struct SagemakerAppImageConfig(std::rc::Rc<SagemakerAppImageConfig_>);

impl SagemakerAppImageConfig {
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

    #[doc = "Set the field `code_editor_app_image_config`.\n"]
    pub fn set_code_editor_app_image_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAppImageConfig_CodeEditorAppImageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code_editor_app_image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.code_editor_app_image_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_image_config`.\n"]
    pub fn set_jupyter_lab_image_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jupyter_lab_image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jupyter_lab_image_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_image_config`.\n"]
    pub fn set_kernel_gateway_image_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAppImageConfig_KernelGatewayImageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kernel_gateway_image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kernel_gateway_image_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_image_config` after provisioning.\n"]
    pub fn code_editor_app_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_CodeEditorAppImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_editor_app_image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_image_config` after provisioning.\n"]
    pub fn jupyter_lab_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_KernelGatewayImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_image_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerAppImageConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerAppImageConfig { }

impl terrars::ToListMappable for SagemakerAppImageConfig {
    type O = terrars::ListRef<SagemakerAppImageConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerAppImageConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_app_image_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerAppImageConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAppImageConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerAppImageConfig {
        let out = SagemakerAppImageConfig(std::rc::Rc::new(SagemakerAppImageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerAppImageConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_image_config_name: self.app_image_config_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                code_editor_app_image_config: core::default::Default::default(),
                jupyter_lab_image_config: core::default::Default::default(),
                kernel_gateway_image_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerAppImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerAppImageConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_image_config` after provisioning.\n"]
    pub fn code_editor_app_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_CodeEditorAppImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_editor_app_image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_image_config` after provisioning.\n"]
    pub fn jupyter_lab_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_image_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_KernelGatewayImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_image_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfigDynamic {
    code_editor_app_image_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_CodeEditorAppImageConfig>>,
    jupyter_lab_image_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig>>,
    kernel_gateway_image_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_KernelGatewayImageConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_arguments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
    #[doc = "Set the field `container_arguments`.\n"]
    pub fn set_container_arguments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_entrypoint`.\n"]
    pub fn set_container_entrypoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_environment_variables`.\n"]
    pub fn set_container_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_environment_variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {}

impl BuildSagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
    pub fn build(self) -> SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
        SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig {
            container_arguments: core::default::Default::default(),
            container_entrypoint: core::default::Default::default(),
            container_environment_variables: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef {
        SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_arguments` after provisioning.\n"]
    pub fn container_arguments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `container_entrypoint` after provisioning.\n"]
    pub fn container_entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_entrypoint", self.base))
    }

    #[doc = "Get a reference to the value of field `container_environment_variables` after provisioning.\n"]
    pub fn container_environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.container_environment_variables", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
    #[doc = "Set the field `default_gid`.\n"]
    pub fn set_default_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_uid`.\n"]
    pub fn set_default_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {}

impl BuildSagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
    pub fn build(self) -> SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
        SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig {
            default_gid: core::default::Default::default(),
            default_uid: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef {
        SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_gid` after provisioning.\n"]
    pub fn default_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_gid", self.base))
    }

    #[doc = "Get a reference to the value of field `default_uid` after provisioning.\n"]
    pub fn default_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_uid", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfig_CodeEditorAppImageConfigDynamic {
    container_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_CodeEditorAppImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    dynamic: SagemakerAppImageConfig_CodeEditorAppImageConfigDynamic,
}

impl SagemakerAppImageConfig_CodeEditorAppImageConfig {
    #[doc = "Set the field `container_config`.\n"]
    pub fn set_container_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_CodeEditorAppImageConfig {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_CodeEditorAppImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_CodeEditorAppImageConfig {}

impl BuildSagemakerAppImageConfig_CodeEditorAppImageConfig {
    pub fn build(self) -> SagemakerAppImageConfig_CodeEditorAppImageConfig {
        SagemakerAppImageConfig_CodeEditorAppImageConfig {
            container_config: core::default::Default::default(),
            file_system_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_CodeEditorAppImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_CodeEditorAppImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_CodeEditorAppImageConfigRef {
        SagemakerAppImageConfig_CodeEditorAppImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_CodeEditorAppImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_config` after provisioning.\n"]
    pub fn container_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfig_JupyterLabImageConfigDynamic {
    container_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    dynamic: SagemakerAppImageConfig_JupyterLabImageConfigDynamic,
}

impl SagemakerAppImageConfig_JupyterLabImageConfig {
    #[doc = "Set the field `container_config`.\n"]
    pub fn set_container_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_JupyterLabImageConfig {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_JupyterLabImageConfig {}

impl BuildSagemakerAppImageConfig_JupyterLabImageConfig {
    pub fn build(self) -> SagemakerAppImageConfig_JupyterLabImageConfig {
        SagemakerAppImageConfig_JupyterLabImageConfig {
            container_config: core::default::Default::default(),
            file_system_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_JupyterLabImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_JupyterLabImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_JupyterLabImageConfigRef {
        SagemakerAppImageConfig_JupyterLabImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_JupyterLabImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_config` after provisioning.\n"]
    pub fn container_config(&self) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfig_ContainerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
    pub fn build(self) -> SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
        SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec {
            display_name: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef {
        SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfig_KernelGatewayImageConfigDynamic {
    file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    kernel_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfig_KernelGatewayImageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_spec: std::option::Option<std::vec::Vec<SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec>>,
    dynamic: SagemakerAppImageConfig_KernelGatewayImageConfigDynamic,
}

impl SagemakerAppImageConfig_KernelGatewayImageConfig {
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_spec`.\n"]
    pub fn set_kernel_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kernel_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfig_KernelGatewayImageConfig {
    type O = terrars::BlockAssignable<SagemakerAppImageConfig_KernelGatewayImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfig_KernelGatewayImageConfig {}

impl BuildSagemakerAppImageConfig_KernelGatewayImageConfig {
    pub fn build(self) -> SagemakerAppImageConfig_KernelGatewayImageConfig {
        SagemakerAppImageConfig_KernelGatewayImageConfig {
            file_system_config: core::default::Default::default(),
            kernel_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAppImageConfig_KernelGatewayImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfig_KernelGatewayImageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfig_KernelGatewayImageConfigRef {
        SagemakerAppImageConfig_KernelGatewayImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfig_KernelGatewayImageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerAppImageConfig_JupyterLabImageConfig_FileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_spec` after provisioning.\n"]
    pub fn kernel_spec(&self) -> terrars::ListRef<SagemakerAppImageConfig_KernelGatewayImageConfig_KernelSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_spec", self.base))
    }
}
