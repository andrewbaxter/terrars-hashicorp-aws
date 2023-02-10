use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerSpaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    space_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_settings: Option<Vec<SagemakerSpaceSpaceSettingsEl>>,
    dynamic: SagemakerSpaceDynamic,
}

struct SagemakerSpace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerSpaceData>,
}

#[derive(Clone)]
pub struct SagemakerSpace(Rc<SagemakerSpace_>);

impl SagemakerSpace {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `space_settings`.\n"]
    pub fn set_space_settings(self, v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }
}

impl Referable for SagemakerSpace {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerSpace { }

impl ToListMappable for SagemakerSpace {
    type O = ListRef<SagemakerSpaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerSpace_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_space".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerSpace {
    pub tf_id: String,
    #[doc= ""]
    pub domain_id: PrimField<String>,
    #[doc= ""]
    pub space_name: PrimField<String>,
}

impl BuildSagemakerSpace {
    pub fn build(self, stack: &mut Stack) -> SagemakerSpace {
        let out = SagemakerSpace(Rc::new(SagemakerSpace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerSpaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                space_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerSpaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerSpaceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc= ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElDynamic {
    jupyter_server_app_settings: Option<DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: Option<DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsEl {
    #[doc= "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerSpaceSpaceSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsEl {}

impl BuildSagemakerSpaceSpaceSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsEl {
        SagemakerSpaceSpaceSettingsEl {
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerSpaceSpaceSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceSpaceSettingsElRef {
        SagemakerSpaceSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceDynamic {
    space_settings: Option<DynamicBlock<SagemakerSpaceSpaceSettingsEl>>,
}
