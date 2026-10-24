use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerModelData {
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
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<SagemakerModelContainerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_execution_config: std::option::Option<std::vec::Vec<SagemakerModelInferenceExecutionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_container: std::option::Option<std::vec::Vec<SagemakerModelPrimaryContainerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerModelVpcConfigEl>>,
    dynamic: SagemakerModelDynamic,
}

struct SagemakerModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerModelData>,
}

#[derive(Clone)]
pub struct SagemakerModel(std::rc::Rc<SagemakerModel_>);

impl SagemakerModel {
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

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(self, v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelContainerEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_execution_config`.\n"]
    pub fn set_inference_execution_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelInferenceExecutionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_execution_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_execution_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary_container`.\n"]
    pub fn set_primary_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelPrimaryContainerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().primary_container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.primary_container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerModelContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(&self) -> terrars::ListRef<SagemakerModelInferenceExecutionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_execution_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<SagemakerModelPrimaryContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerModelVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerModel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerModel { }

impl terrars::ToListMappable for SagemakerModel {
    type O = terrars::ListRef<SagemakerModelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerModel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_model".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerModel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerModel {
        let out = SagemakerModel(std::rc::Rc::new(SagemakerModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerModelData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enable_network_isolation: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                container: core::default::Default::default(),
                inference_execution_config: core::default::Default::default(),
                primary_container: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerModelRef {
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

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerModelContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(&self) -> terrars::ListRef<SagemakerModelInferenceExecutionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_execution_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<SagemakerModelPrimaryContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerModelVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl { }

impl terrars::ToListMappable for SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    #[doc = ""]
    pub repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    pub fn build(self) -> SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
        SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
            repository_credentials_provider_arn: self.repository_credentials_provider_arn,
        }
    }
}

pub struct SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
        SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_credentials_provider_arn` after provisioning.\n"]
    pub fn repository_credentials_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_credentials_provider_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelContainerElImageConfigElDynamic {
    repository_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerModelContainerElImageConfigEl {
    repository_access_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_auth_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
    dynamic: SagemakerModelContainerElImageConfigElDynamic,
}

impl SagemakerModelContainerElImageConfigEl {
    #[doc = "Set the field `repository_auth_config`.\n"]
    pub fn set_repository_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModelContainerElImageConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelContainerElImageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelContainerElImageConfigEl {
    #[doc = ""]
    pub repository_access_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelContainerElImageConfigEl {
    pub fn build(self) -> SagemakerModelContainerElImageConfigEl {
        SagemakerModelContainerElImageConfigEl {
            repository_access_mode: self.repository_access_mode,
            repository_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerModelContainerElImageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelContainerElImageConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelContainerElImageConfigElRef {
        SagemakerModelContainerElImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelContainerElImageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_access_mode` after provisioning.\n"]
    pub fn repository_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_auth_config` after provisioning.\n"]
    pub fn repository_auth_config(
        &self,
    ) -> terrars::ListRef<SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelContainerElDynamic {
    image_config: std::option::Option<terrars::DynamicBlock<SagemakerModelContainerElImageConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerModelContainerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<SagemakerModelContainerElImageConfigEl>>,
    dynamic: SagemakerModelContainerElDynamic,
}

impl SagemakerModelContainerEl {
    #[doc = "Set the field `container_hostname`.\n"]
    pub fn set_container_hostname(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_url`.\n"]
    pub fn set_model_data_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_data_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelContainerElImageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModelContainerEl {
    type O = terrars::BlockAssignable<SagemakerModelContainerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelContainerEl {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelContainerEl {
    pub fn build(self) -> SagemakerModelContainerEl {
        SagemakerModelContainerEl {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: self.image,
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            image_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerModelContainerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelContainerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelContainerElRef {
        SagemakerModelContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelContainerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_hostname` after provisioning.\n"]
    pub fn container_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_url", self.base))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<SagemakerModelContainerElImageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelInferenceExecutionConfigEl {
    mode: terrars::PrimField<std::string::String>,
}

impl SagemakerModelInferenceExecutionConfigEl { }

impl terrars::ToListMappable for SagemakerModelInferenceExecutionConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelInferenceExecutionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelInferenceExecutionConfigEl {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelInferenceExecutionConfigEl {
    pub fn build(self) -> SagemakerModelInferenceExecutionConfigEl {
        SagemakerModelInferenceExecutionConfigEl { mode: self.mode }
    }
}

pub struct SagemakerModelInferenceExecutionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelInferenceExecutionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelInferenceExecutionConfigElRef {
        SagemakerModelInferenceExecutionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelInferenceExecutionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl { }

impl terrars::ToListMappable for SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    #[doc = ""]
    pub repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
        SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
            repository_credentials_provider_arn: self.repository_credentials_provider_arn,
        }
    }
}

pub struct SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
        SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_credentials_provider_arn` after provisioning.\n"]
    pub fn repository_credentials_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_credentials_provider_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElImageConfigElDynamic {
    repository_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElImageConfigEl {
    repository_access_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_auth_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>>,
    dynamic: SagemakerModelPrimaryContainerElImageConfigElDynamic,
}

impl SagemakerModelPrimaryContainerElImageConfigEl {
    #[doc = "Set the field `repository_auth_config`.\n"]
    pub fn set_repository_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModelPrimaryContainerElImageConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelPrimaryContainerElImageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelPrimaryContainerElImageConfigEl {
    #[doc = ""]
    pub repository_access_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelPrimaryContainerElImageConfigEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElImageConfigEl {
        SagemakerModelPrimaryContainerElImageConfigEl {
            repository_access_mode: self.repository_access_mode,
            repository_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerModelPrimaryContainerElImageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelPrimaryContainerElImageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModelPrimaryContainerElImageConfigElRef {
        SagemakerModelPrimaryContainerElImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelPrimaryContainerElImageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_access_mode` after provisioning.\n"]
    pub fn repository_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_auth_config` after provisioning.\n"]
    pub fn repository_auth_config(
        &self,
    ) -> terrars::ListRef<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElDynamic {
    image_config: std::option::Option<terrars::DynamicBlock<SagemakerModelPrimaryContainerElImageConfigEl>>,
}

#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<SagemakerModelPrimaryContainerElImageConfigEl>>,
    dynamic: SagemakerModelPrimaryContainerElDynamic,
}

impl SagemakerModelPrimaryContainerEl {
    #[doc = "Set the field `container_hostname`.\n"]
    pub fn set_container_hostname(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_url`.\n"]
    pub fn set_model_data_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_data_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelPrimaryContainerElImageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModelPrimaryContainerEl {
    type O = terrars::BlockAssignable<SagemakerModelPrimaryContainerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelPrimaryContainerEl {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelPrimaryContainerEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerEl {
        SagemakerModelPrimaryContainerEl {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: self.image,
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            image_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerModelPrimaryContainerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelPrimaryContainerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelPrimaryContainerElRef {
        SagemakerModelPrimaryContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelPrimaryContainerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_hostname` after provisioning.\n"]
    pub fn container_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_url", self.base))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<SagemakerModelPrimaryContainerElImageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelVpcConfigEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerModelVpcConfigEl { }

impl terrars::ToListMappable for SagemakerModelVpcConfigEl {
    type O = terrars::BlockAssignable<SagemakerModelVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerModelVpcConfigEl {
    pub fn build(self) -> SagemakerModelVpcConfigEl {
        SagemakerModelVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

pub struct SagemakerModelVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelVpcConfigElRef {
        SagemakerModelVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelVpcConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelDynamic {
    container: std::option::Option<terrars::DynamicBlock<SagemakerModelContainerEl>>,
    inference_execution_config: std::option::Option<terrars::DynamicBlock<SagemakerModelInferenceExecutionConfigEl>>,
    primary_container: std::option::Option<terrars::DynamicBlock<SagemakerModelPrimaryContainerEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<SagemakerModelVpcConfigEl>>,
}
