use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerSpaceData {
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
    domain_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    space_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_settings: std::option::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsEl>>,
    dynamic: SagemakerSpaceDynamic,
}

struct SagemakerSpace_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerSpaceData>,
}

#[derive(Clone)]
pub struct SagemakerSpace(std::rc::Rc<SagemakerSpace_>);

impl SagemakerSpace {
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

    #[doc = "Set the field `space_settings`.\n"]
    pub fn set_space_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpaceSpaceSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerSpace {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerSpace { }

impl terrars::ToListMappable for SagemakerSpace {
    type O = terrars::ListRef<SagemakerSpaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerSpace_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_space".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerSpace {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub space_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerSpace {
        let out = SagemakerSpace(std::rc::Rc::new(SagemakerSpace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerSpaceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                space_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerSpaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerSpaceRef {
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElDynamic {
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElDynamic,
}

impl SagemakerSpaceSpaceSettingsEl {
    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpaceSpaceSettingsEl {
    type O = terrars::BlockAssignable<SagemakerSpaceSpaceSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerSpaceSpaceSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceSpaceSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerSpaceSpaceSettingsElRef {
        SagemakerSpaceSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpaceSpaceSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceDynamic {
    space_settings: std::option::Option<terrars::DynamicBlock<SagemakerSpaceSpaceSettingsEl>>,
}
