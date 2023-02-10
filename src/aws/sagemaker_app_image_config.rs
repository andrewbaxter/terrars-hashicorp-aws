use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerAppImageConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_image_config_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_image_config: Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
    dynamic: SagemakerAppImageConfigDynamic,
}

struct SagemakerAppImageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerAppImageConfigData>,
}

#[derive(Clone)]
pub struct SagemakerAppImageConfig(Rc<SagemakerAppImageConfig_>);

impl SagemakerAppImageConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `kernel_gateway_image_config`.\n"]
    pub fn set_kernel_gateway_image_config(
        self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kernel_gateway_image_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kernel_gateway_image_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_image_config", self.extract_ref()))
    }
}

impl Resource for SagemakerAppImageConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SagemakerAppImageConfig {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SagemakerAppImageConfig {
    type O = ListRef<SagemakerAppImageConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for SagemakerAppImageConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_app_image_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerAppImageConfig {
    pub tf_id: String,
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
}

impl BuildSagemakerAppImageConfig {
    pub fn build(self, stack: &mut Stack) -> SagemakerAppImageConfig {
        let out = SagemakerAppImageConfig(Rc::new(SagemakerAppImageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerAppImageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_image_config_name: self.app_image_config_name,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                kernel_gateway_image_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerAppImageConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerAppImageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerAppImageConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(&self) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_image_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    #[doc= "Set the field `default_gid`.\n"]
    pub fn set_default_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_gid = Some(v.into());
        self
    }

    #[doc= "Set the field `default_uid`.\n"]
    pub fn set_default_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_uid = Some(v.into());
        self
    }

    #[doc= "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_gid` after provisioning.\n"]
    pub fn default_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_gid", self.base))
    }

    #[doc= "Get a reference to the value of field `default_uid` after provisioning.\n"]
    pub fn default_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_uid", self.base))
    }

    #[doc= "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc= "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc= ""]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    fn new(shared: StackShared, base: String) -> SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfigKernelGatewayImageConfigElDynamic {
    file_system_config: Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    kernel_spec: Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_config: Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_spec: Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
    dynamic: SagemakerAppImageConfigKernelGatewayImageConfigElDynamic,
}

impl SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[doc= "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_system_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kernel_spec`.\n"]
    pub fn set_kernel_spec(
        mut self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerAppImageConfigKernelGatewayImageConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_system_config", self.base))
    }

    #[doc= "Get a reference to the value of field `kernel_spec` after provisioning.\n"]
    pub fn kernel_spec(&self) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAppImageConfigDynamic {
    kernel_gateway_image_config: Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
}
