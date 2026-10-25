#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerModelData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<SagemakerModel_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_execution_config: std::option::Option<std::vec::Vec<SagemakerModel_InferenceExecutionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_container: std::option::Option<std::vec::Vec<SagemakerModel_PrimaryContainer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerModel_VpcConfig>>,
    dynamic: SagemakerModelDynamic,
}

struct SagemakerModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerModelData_>,
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

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(self, v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_InferenceExecutionConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_PrimaryContainer>>,
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
    pub fn set_vpc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_VpcConfig>>) -> Self {
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

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerModel_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(&self) -> terrars::ListRef<SagemakerModel_InferenceExecutionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_execution_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<SagemakerModel_PrimaryContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerModel_VpcConfigRef> {
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
            data: std::cell::RefCell::new(SagemakerModelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enable_network_isolation: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerModel_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(&self) -> terrars::ListRef<SagemakerModel_InferenceExecutionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_execution_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<SagemakerModel_PrimaryContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerModel_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelDynamic {
    container: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container>>,
    inference_execution_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_InferenceExecutionConfig>>,
    primary_container: std::option::Option<terrars::DynamicBlock<SagemakerModel_PrimaryContainer>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_VpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig {
    accept_eula: terrars::PrimField<bool>,
}

impl SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig { }

impl terrars::ToListMappable for SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig {
    type O = terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig {
    #[doc = ""]
    pub accept_eula: terrars::PrimField<bool>,
}

impl BuildSagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig {
    pub fn build(self) -> SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig {
        SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig { accept_eula: self.accept_eula }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef {
        SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_Container_ModelDataSource_S3DataSourceDynamic {
    model_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_ModelDataSource_S3DataSource {
    compression_type: terrars::PrimField<std::string::String>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_access_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig>>,
    dynamic: SagemakerModel_Container_ModelDataSource_S3DataSourceDynamic,
}

impl SagemakerModel_Container_ModelDataSource_S3DataSource {
    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_Container_ModelDataSource_S3DataSource {
    type O = terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource_S3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_ModelDataSource_S3DataSource {
    #[doc = ""]
    pub compression_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel_Container_ModelDataSource_S3DataSource {
    pub fn build(self) -> SagemakerModel_Container_ModelDataSource_S3DataSource {
        SagemakerModel_Container_ModelDataSource_S3DataSource {
            compression_type: self.compression_type,
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            model_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_ModelDataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_ModelDataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModel_Container_ModelDataSource_S3DataSourceRef {
        SagemakerModel_Container_ModelDataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_ModelDataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> terrars::ListRef<SagemakerModel_Container_ModelDataSource_S3DataSource_ModelAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_access_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_Container_AdditionalModelDataSourceDynamic {
    s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_AdditionalModelDataSource {
    channel_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
    dynamic: SagemakerModel_Container_AdditionalModelDataSourceDynamic,
}

impl SagemakerModel_Container_AdditionalModelDataSource {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_Container_AdditionalModelDataSource {
    type O = terrars::BlockAssignable<SagemakerModel_Container_AdditionalModelDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_AdditionalModelDataSource {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel_Container_AdditionalModelDataSource {
    pub fn build(self) -> SagemakerModel_Container_AdditionalModelDataSource {
        SagemakerModel_Container_AdditionalModelDataSource {
            channel_name: self.channel_name,
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_AdditionalModelDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_AdditionalModelDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModel_Container_AdditionalModelDataSourceRef {
        SagemakerModel_Container_AdditionalModelDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_AdditionalModelDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(&self) -> terrars::ListRef<SagemakerModel_Container_ModelDataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
    repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl SagemakerModel_Container_ImageConfig_RepositoryAuthConfig { }

impl terrars::ToListMappable for SagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
    type O = terrars::BlockAssignable<SagemakerModel_Container_ImageConfig_RepositoryAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
    #[doc = ""]
    pub repository_credentials_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
    pub fn build(self) -> SagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
        SagemakerModel_Container_ImageConfig_RepositoryAuthConfig {
            repository_credentials_provider_arn: self.repository_credentials_provider_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef {
        SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_credentials_provider_arn` after provisioning.\n"]
    pub fn repository_credentials_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_credentials_provider_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_Container_ImageConfigDynamic {
    repository_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_ImageConfig_RepositoryAuthConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_ImageConfig {
    repository_access_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_auth_config: std::option::Option<std::vec::Vec<SagemakerModel_Container_ImageConfig_RepositoryAuthConfig>>,
    dynamic: SagemakerModel_Container_ImageConfigDynamic,
}

impl SagemakerModel_Container_ImageConfig {
    #[doc = "Set the field `repository_auth_config`.\n"]
    pub fn set_repository_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerModel_Container_ImageConfig_RepositoryAuthConfig>>,
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

impl terrars::ToListMappable for SagemakerModel_Container_ImageConfig {
    type O = terrars::BlockAssignable<SagemakerModel_Container_ImageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_ImageConfig {
    #[doc = ""]
    pub repository_access_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel_Container_ImageConfig {
    pub fn build(self) -> SagemakerModel_Container_ImageConfig {
        SagemakerModel_Container_ImageConfig {
            repository_access_mode: self.repository_access_mode,
            repository_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_ImageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_ImageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_Container_ImageConfigRef {
        SagemakerModel_Container_ImageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_ImageConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<SagemakerModel_Container_ImageConfig_RepositoryAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_Container_ModelDataSourceDynamic {
    s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_ModelDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
    dynamic: SagemakerModel_Container_ModelDataSourceDynamic,
}

impl SagemakerModel_Container_ModelDataSource {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource_S3DataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_Container_ModelDataSource {
    type O = terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_ModelDataSource {}

impl BuildSagemakerModel_Container_ModelDataSource {
    pub fn build(self) -> SagemakerModel_Container_ModelDataSource {
        SagemakerModel_Container_ModelDataSource {
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_ModelDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_ModelDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_Container_ModelDataSourceRef {
        SagemakerModel_Container_ModelDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_ModelDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(&self) -> terrars::ListRef<SagemakerModel_Container_ModelDataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModel_Container_MultiModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_cache_setting: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerModel_Container_MultiModelConfig {
    #[doc = "Set the field `model_cache_setting`.\n"]
    pub fn set_model_cache_setting(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_cache_setting = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_Container_MultiModelConfig {
    type O = terrars::BlockAssignable<SagemakerModel_Container_MultiModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container_MultiModelConfig {}

impl BuildSagemakerModel_Container_MultiModelConfig {
    pub fn build(self) -> SagemakerModel_Container_MultiModelConfig {
        SagemakerModel_Container_MultiModelConfig { model_cache_setting: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_Container_MultiModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_Container_MultiModelConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_Container_MultiModelConfigRef {
        SagemakerModel_Container_MultiModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_Container_MultiModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_cache_setting` after provisioning.\n"]
    pub fn model_cache_setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_cache_setting", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_ContainerDynamic {
    additional_model_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_AdditionalModelDataSource>>,
    image_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_ImageConfig>>,
    model_data_source: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_ModelDataSource>>,
    multi_model_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_MultiModelConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_specification_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_package_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_model_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_AdditionalModelDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<SagemakerModel_Container_ImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_ModelDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_model_config: std::option::Option<std::vec::Vec<SagemakerModel_Container_MultiModelConfig>>,
    dynamic: SagemakerModel_ContainerDynamic,
}

impl SagemakerModel_Container {
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

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_specification_name`.\n"]
    pub fn set_inference_specification_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_specification_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `model_package_name`.\n"]
    pub fn set_model_package_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_package_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_model_data_source`.\n"]
    pub fn set_additional_model_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_AdditionalModelDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_model_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_model_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ImageConfig>>,
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

    #[doc = "Set the field `model_data_source`.\n"]
    pub fn set_model_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_model_config`.\n"]
    pub fn set_multi_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_MultiModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_Container {
    type O = terrars::BlockAssignable<SagemakerModel_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_Container {}

impl BuildSagemakerModel_Container {
    pub fn build(self) -> SagemakerModel_Container {
        SagemakerModel_Container {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: core::default::Default::default(),
            inference_specification_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            model_package_name: core::default::Default::default(),
            additional_model_data_source: core::default::Default::default(),
            image_config: core::default::Default::default(),
            model_data_source: core::default::Default::default(),
            multi_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_ContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_ContainerRef {
        SagemakerModel_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_ContainerRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `inference_specification_name` after provisioning.\n"]
    pub fn inference_specification_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_specification_name", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_url", self.base))
    }

    #[doc = "Get a reference to the value of field `model_package_name` after provisioning.\n"]
    pub fn model_package_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_package_name", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_model_data_source` after provisioning.\n"]
    pub fn additional_model_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerModel_Container_AdditionalModelDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_model_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<SagemakerModel_Container_ImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_source` after provisioning.\n"]
    pub fn model_data_source(&self) -> terrars::ListRef<SagemakerModel_Container_ModelDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_model_config` after provisioning.\n"]
    pub fn multi_model_config(&self) -> terrars::ListRef<SagemakerModel_Container_MultiModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModel_InferenceExecutionConfig {
    mode: terrars::PrimField<std::string::String>,
}

impl SagemakerModel_InferenceExecutionConfig { }

impl terrars::ToListMappable for SagemakerModel_InferenceExecutionConfig {
    type O = terrars::BlockAssignable<SagemakerModel_InferenceExecutionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_InferenceExecutionConfig {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModel_InferenceExecutionConfig {
    pub fn build(self) -> SagemakerModel_InferenceExecutionConfig {
        SagemakerModel_InferenceExecutionConfig { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_InferenceExecutionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_InferenceExecutionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_InferenceExecutionConfigRef {
        SagemakerModel_InferenceExecutionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_InferenceExecutionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerModel_PrimaryContainerDynamic {
    additional_model_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerModel_Container_AdditionalModelDataSource>>,
    image_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_ImageConfig>>,
    model_data_source: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_ModelDataSource>>,
    multi_model_config: std::option::Option<terrars::DynamicBlock<SagemakerModel_Container_MultiModelConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModel_PrimaryContainer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_specification_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_package_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_model_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_AdditionalModelDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_config: std::option::Option<std::vec::Vec<SagemakerModel_Container_ImageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_source: std::option::Option<std::vec::Vec<SagemakerModel_Container_ModelDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_model_config: std::option::Option<std::vec::Vec<SagemakerModel_Container_MultiModelConfig>>,
    dynamic: SagemakerModel_PrimaryContainerDynamic,
}

impl SagemakerModel_PrimaryContainer {
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

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_specification_name`.\n"]
    pub fn set_inference_specification_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_specification_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `model_package_name`.\n"]
    pub fn set_model_package_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_package_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_model_data_source`.\n"]
    pub fn set_additional_model_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_AdditionalModelDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_model_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_model_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ImageConfig>>,
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

    #[doc = "Set the field `model_data_source`.\n"]
    pub fn set_model_data_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_ModelDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_model_config`.\n"]
    pub fn set_multi_model_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModel_Container_MultiModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerModel_PrimaryContainer {
    type O = terrars::BlockAssignable<SagemakerModel_PrimaryContainer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_PrimaryContainer {}

impl BuildSagemakerModel_PrimaryContainer {
    pub fn build(self) -> SagemakerModel_PrimaryContainer {
        SagemakerModel_PrimaryContainer {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: core::default::Default::default(),
            inference_specification_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            model_package_name: core::default::Default::default(),
            additional_model_data_source: core::default::Default::default(),
            image_config: core::default::Default::default(),
            model_data_source: core::default::Default::default(),
            multi_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_PrimaryContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_PrimaryContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_PrimaryContainerRef {
        SagemakerModel_PrimaryContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_PrimaryContainerRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `inference_specification_name` after provisioning.\n"]
    pub fn inference_specification_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_specification_name", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_url", self.base))
    }

    #[doc = "Get a reference to the value of field `model_package_name` after provisioning.\n"]
    pub fn model_package_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_package_name", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_model_data_source` after provisioning.\n"]
    pub fn additional_model_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerModel_Container_AdditionalModelDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_model_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> terrars::ListRef<SagemakerModel_Container_ImageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_source` after provisioning.\n"]
    pub fn model_data_source(&self) -> terrars::ListRef<SagemakerModel_Container_ModelDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_model_config` after provisioning.\n"]
    pub fn multi_model_config(&self) -> terrars::ListRef<SagemakerModel_Container_MultiModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModel_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerModel_VpcConfig { }

impl terrars::ToListMappable for SagemakerModel_VpcConfig {
    type O = terrars::BlockAssignable<SagemakerModel_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModel_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerModel_VpcConfig {
    pub fn build(self) -> SagemakerModel_VpcConfig {
        SagemakerModel_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerModel_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModel_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModel_VpcConfigRef {
        SagemakerModel_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModel_VpcConfigRef {
    #[allow(dead_code)]
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
