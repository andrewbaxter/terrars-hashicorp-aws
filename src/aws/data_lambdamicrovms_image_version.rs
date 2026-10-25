#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLambdamicrovmsImageVersionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    image_identifier: terrars::PrimField<std::string::String>,
    image_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLambdamicrovmsImageVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLambdamicrovmsImageVersionData_>,
}

#[derive(Clone)]
pub struct DataLambdamicrovmsImageVersion(std::rc::Rc<DataLambdamicrovmsImageVersion_>);

impl DataLambdamicrovmsImageVersion {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `additional_os_capabilities` after provisioning.\n"]
    pub fn additional_os_capabilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_os_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_version` after provisioning.\n"]
    pub fn base_image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_role_arn` after provisioning.\n"]
    pub fn build_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_artifact` after provisioning.\n"]
    pub fn code_artifact(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_CodeArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_configuration` after provisioning.\n"]
    pub fn cpu_configuration(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_CpuConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_network_connectors` after provisioning.\n"]
    pub fn egress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hooks` after provisioning.\n"]
    pub fn hooks(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_HooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_identifier` after provisioning.\n"]
    pub fn image_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataLambdamicrovmsImageVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLambdamicrovmsImageVersion { }

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion {
    type O = terrars::ListRef<DataLambdamicrovmsImageVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLambdamicrovmsImageVersion_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lambdamicrovms_image_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLambdamicrovmsImageVersion {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub image_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_version: terrars::PrimField<std::string::String>,
}

impl BuildDataLambdamicrovmsImageVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLambdamicrovmsImageVersion {
        let out = DataLambdamicrovmsImageVersion(std::rc::Rc::new(DataLambdamicrovmsImageVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLambdamicrovmsImageVersionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                image_identifier: self.image_identifier,
                image_version: self.image_version,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLambdamicrovmsImageVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLambdamicrovmsImageVersionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `additional_os_capabilities` after provisioning.\n"]
    pub fn additional_os_capabilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_os_capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_version` after provisioning.\n"]
    pub fn base_image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_role_arn` after provisioning.\n"]
    pub fn build_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_artifact` after provisioning.\n"]
    pub fn code_artifact(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_CodeArtifactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_artifact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_configuration` after provisioning.\n"]
    pub fn cpu_configuration(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_CpuConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_network_connectors` after provisioning.\n"]
    pub fn egress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hooks` after provisioning.\n"]
    pub fn hooks(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_HooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_identifier` after provisioning.\n"]
    pub fn image_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_CodeArtifact {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdamicrovmsImageVersion_CodeArtifact {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_CodeArtifact {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_CodeArtifact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_CodeArtifact {}

impl BuildDataLambdamicrovmsImageVersion_CodeArtifact {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_CodeArtifact {
        DataLambdamicrovmsImageVersion_CodeArtifact { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_CodeArtifactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_CodeArtifactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_CodeArtifactRef {
        DataLambdamicrovmsImageVersion_CodeArtifactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_CodeArtifactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_CpuConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdamicrovmsImageVersion_CpuConfiguration {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_CpuConfiguration {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_CpuConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_CpuConfiguration {}

impl BuildDataLambdamicrovmsImageVersion_CpuConfiguration {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_CpuConfiguration {
        DataLambdamicrovmsImageVersion_CpuConfiguration { architecture: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_CpuConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_CpuConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_CpuConfigurationRef {
        DataLambdamicrovmsImageVersion_CpuConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_CpuConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resume: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resume_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspend: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspend_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
    #[doc = "Set the field `resume`.\n"]
    pub fn set_resume(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resume_timeout_in_seconds`.\n"]
    pub fn set_resume_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.resume_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run`.\n"]
    pub fn set_run(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_timeout_in_seconds`.\n"]
    pub fn set_run_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspend`.\n"]
    pub fn set_suspend(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suspend = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspend_timeout_in_seconds`.\n"]
    pub fn set_suspend_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.suspend_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminate`.\n"]
    pub fn set_terminate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.terminate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terminate_timeout_in_seconds`.\n"]
    pub fn set_terminate_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.terminate_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {}

impl BuildDataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
        DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks {
            resume: core::default::Default::default(),
            resume_timeout_in_seconds: core::default::Default::default(),
            run: core::default::Default::default(),
            run_timeout_in_seconds: core::default::Default::default(),
            suspend: core::default::Default::default(),
            suspend_timeout_in_seconds: core::default::Default::default(),
            terminate: core::default::Default::default(),
            terminate_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef {
        DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resume` after provisioning.\n"]
    pub fn resume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resume", self.base))
    }

    #[doc = "Get a reference to the value of field `resume_timeout_in_seconds` after provisioning.\n"]
    pub fn resume_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resume_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run", self.base))
    }

    #[doc = "Get a reference to the value of field `run_timeout_in_seconds` after provisioning.\n"]
    pub fn run_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `suspend` after provisioning.\n"]
    pub fn suspend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspend", self.base))
    }

    #[doc = "Get a reference to the value of field `suspend_timeout_in_seconds` after provisioning.\n"]
    pub fn suspend_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspend_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `terminate` after provisioning.\n"]
    pub fn terminate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminate", self.base))
    }

    #[doc = "Get a reference to the value of field `terminate_timeout_in_seconds` after provisioning.\n"]
    pub fn terminate_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminate_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ready: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ready_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
    #[doc = "Set the field `ready`.\n"]
    pub fn set_ready(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ready = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ready_timeout_in_seconds`.\n"]
    pub fn set_ready_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ready_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validate`.\n"]
    pub fn set_validate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.validate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validate_timeout_in_seconds`.\n"]
    pub fn set_validate_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.validate_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {}

impl BuildDataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
        DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks {
            ready: core::default::Default::default(),
            ready_timeout_in_seconds: core::default::Default::default(),
            validate: core::default::Default::default(),
            validate_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef {
        DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ready` after provisioning.\n"]
    pub fn ready(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ready", self.base))
    }

    #[doc = "Get a reference to the value of field `ready_timeout_in_seconds` after provisioning.\n"]
    pub fn ready_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ready_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `validate` after provisioning.\n"]
    pub fn validate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validate", self.base))
    }

    #[doc = "Get a reference to the value of field `validate_timeout_in_seconds` after provisioning.\n"]
    pub fn validate_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validate_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Hooks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microvm_hooks: std::option::Option<terrars::ListField<DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microvm_image_hooks: std::option::Option<terrars::ListField<DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdamicrovmsImageVersion_Hooks {
    #[doc = "Set the field `microvm_hooks`.\n"]
    pub fn set_microvm_hooks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLambdamicrovmsImageVersion_Hooks_MicrovmHooks>>,
    ) -> Self {
        self.microvm_hooks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `microvm_image_hooks`.\n"]
    pub fn set_microvm_image_hooks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooks>>,
    ) -> Self {
        self.microvm_image_hooks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Hooks {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Hooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Hooks {}

impl BuildDataLambdamicrovmsImageVersion_Hooks {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Hooks {
        DataLambdamicrovmsImageVersion_Hooks {
            microvm_hooks: core::default::Default::default(),
            microvm_image_hooks: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_HooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_HooksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdamicrovmsImageVersion_HooksRef {
        DataLambdamicrovmsImageVersion_HooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_HooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `microvm_hooks` after provisioning.\n"]
    pub fn microvm_hooks(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_Hooks_MicrovmHooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microvm_hooks", self.base))
    }

    #[doc = "Get a reference to the value of field `microvm_image_hooks` after provisioning.\n"]
    pub fn microvm_image_hooks(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_Hooks_MicrovmImageHooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microvm_image_hooks", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Logging_Cloudwatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdamicrovmsImageVersion_Logging_Cloudwatch {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream`.\n"]
    pub fn set_log_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Logging_Cloudwatch {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Logging_Cloudwatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Logging_Cloudwatch {}

impl BuildDataLambdamicrovmsImageVersion_Logging_Cloudwatch {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Logging_Cloudwatch {
        DataLambdamicrovmsImageVersion_Logging_Cloudwatch {
            log_group: core::default::Default::default(),
            log_stream: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_Logging_CloudwatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_Logging_CloudwatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_Logging_CloudwatchRef {
        DataLambdamicrovmsImageVersion_Logging_CloudwatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_Logging_CloudwatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream` after provisioning.\n"]
    pub fn log_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Logging_Disabled {}

impl DataLambdamicrovmsImageVersion_Logging_Disabled { }

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Logging_Disabled {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Logging_Disabled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Logging_Disabled {}

impl BuildDataLambdamicrovmsImageVersion_Logging_Disabled {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Logging_Disabled {
        DataLambdamicrovmsImageVersion_Logging_Disabled {}
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_Logging_DisabledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_Logging_DisabledRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdamicrovmsImageVersion_Logging_DisabledRef {
        DataLambdamicrovmsImageVersion_Logging_DisabledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_Logging_DisabledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch: std::option::Option<terrars::ListField<DataLambdamicrovmsImageVersion_Logging_Cloudwatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::ListField<DataLambdamicrovmsImageVersion_Logging_Disabled>>,
}

impl DataLambdamicrovmsImageVersion_Logging {
    #[doc = "Set the field `cloudwatch`.\n"]
    pub fn set_cloudwatch(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLambdamicrovmsImageVersion_Logging_Cloudwatch>>,
    ) -> Self {
        self.cloudwatch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLambdamicrovmsImageVersion_Logging_Disabled>>,
    ) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Logging {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Logging {}

impl BuildDataLambdamicrovmsImageVersion_Logging {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Logging {
        DataLambdamicrovmsImageVersion_Logging {
            cloudwatch: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdamicrovmsImageVersion_LoggingRef {
        DataLambdamicrovmsImageVersion_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch` after provisioning.\n"]
    pub fn cloudwatch(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_Logging_CloudwatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<DataLambdamicrovmsImageVersion_Logging_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdamicrovmsImageVersion_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_memory_in_mib: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLambdamicrovmsImageVersion_Resources {
    #[doc = "Set the field `minimum_memory_in_mib`.\n"]
    pub fn set_minimum_memory_in_mib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_memory_in_mib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdamicrovmsImageVersion_Resources {
    type O = terrars::BlockAssignable<DataLambdamicrovmsImageVersion_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdamicrovmsImageVersion_Resources {}

impl BuildDataLambdamicrovmsImageVersion_Resources {
    pub fn build(self) -> DataLambdamicrovmsImageVersion_Resources {
        DataLambdamicrovmsImageVersion_Resources { minimum_memory_in_mib: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdamicrovmsImageVersion_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdamicrovmsImageVersion_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdamicrovmsImageVersion_ResourcesRef {
        DataLambdamicrovmsImageVersion_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdamicrovmsImageVersion_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minimum_memory_in_mib` after provisioning.\n"]
    pub fn minimum_memory_in_mib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_memory_in_mib", self.base))
    }
}
