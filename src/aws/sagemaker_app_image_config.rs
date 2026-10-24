use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerAppImageConfigData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_image_config: std::option::Option<std::vec::Vec<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
    dynamic: SagemakerAppImageConfigDynamic,
}

struct SagemakerAppImageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerAppImageConfigData>,
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

    #[doc = "Set the field `kernel_gateway_image_config`.\n"]
    pub fn set_kernel_gateway_image_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
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
            data: std::cell::RefCell::new(SagemakerAppImageConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_image_config_name: self.app_image_config_name,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> terrars::ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_image_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
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

impl terrars::ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    type O = terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {}

impl BuildSagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
        SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
            default_gid: core::default::Default::default(),
            default_uid: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
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

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    type O = terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
        SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
            display_name: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
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
struct SagemakerAppImageConfigKernelGatewayImageConfigElDynamic {
    file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    kernel_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_spec: std::option::Option<std::vec::Vec<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
    dynamic: SagemakerAppImageConfigKernelGatewayImageConfigElDynamic,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>,
                    >,
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
                    ::Into<terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
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

impl terrars::ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigEl {
    type O = terrars::BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigEl {}

impl BuildSagemakerAppImageConfigKernelGatewayImageConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigEl {
        SagemakerAppImageConfigKernelGatewayImageConfigEl {
            file_system_config: core::default::Default::default(),
            kernel_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_spec` after provisioning.\n"]
    pub fn kernel_spec(&self) -> terrars::ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfigDynamic {
    kernel_gateway_image_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
}
