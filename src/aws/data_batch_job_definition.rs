#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBatchJobDefinitionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBatchJobDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBatchJobDefinitionData_>,
}

#[derive(Clone)]
pub struct DataBatchJobDefinition(std::rc::Rc<DataBatchJobDefinition_>);

impl DataBatchJobDefinition {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `container_orchestration_type` after provisioning.\n"]
    pub fn container_orchestration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_orchestration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> terrars::ListRef<DataBatchJobDefinition_EksPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_properties", self.extract_ref()))
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
    pub fn node_properties(&self) -> terrars::ListRef<DataBatchJobDefinition_NodePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<DataBatchJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataBatchJobDefinition_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataBatchJobDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBatchJobDefinition { }

impl terrars::ToListMappable for DataBatchJobDefinition {
    type O = terrars::ListRef<DataBatchJobDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBatchJobDefinition_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_batch_job_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBatchJobDefinition {
    pub tf_id: std::string::String,
}

impl BuildDataBatchJobDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBatchJobDefinition {
        let out = DataBatchJobDefinition(std::rc::Rc::new(DataBatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBatchJobDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                revision: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBatchJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBatchJobDefinitionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_orchestration_type` after provisioning.\n"]
    pub fn container_orchestration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_orchestration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> terrars::ListRef<DataBatchJobDefinition_EksPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_properties", self.extract_ref()))
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
    pub fn node_properties(&self) -> terrars::ListRef<DataBatchJobDefinition_NodePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<DataBatchJobDefinition_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataBatchJobDefinition_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
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

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
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

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
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

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
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

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef {
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

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_context: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Containers {
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

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources>,
                    >,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
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
                        ::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>,
                    >,
    ) -> Self {
        self.security_context = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>,
                    >,
    ) -> Self {
        self.volume_mounts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Containers {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Containers {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Containers {
        DataBatchJobDefinition_EksProperties_PodProperties_Containers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef {
        DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef {
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

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
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
    pub fn resources(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_context", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
        DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef {
        DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_context: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
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

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Env>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_Resources>,
                    >,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
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
                        ::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContext>,
                    >,
    ) -> Self {
        self.security_context = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMounts>,
                    >,
    ) -> Self {
        self.volume_mounts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_InitContainers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_InitContainers {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
        DataBatchJobDefinition_EksProperties_PodProperties_InitContainers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
        DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef {
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

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_EnvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
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
    pub fn resources(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_SecurityContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_context", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Metadata {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Metadata {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Metadata {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Metadata {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Metadata {
        DataBatchJobDefinition_EksProperties_PodProperties_Metadata { labels: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef {
        DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_limit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_limit`.\n"]
    pub fn set_size_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath {
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef {
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
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    #[doc = "Set the field `optional`.\n"]
    pub fn set_optional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.optional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret {
            optional: core::default::Default::default(),
            secret_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef {
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

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_dir: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties_Volumes {
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDir>>,
    ) -> Self {
        self.empty_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPath>>,
    ) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_Secret>>,
    ) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties_Volumes {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties_Volumes {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties_Volumes {
        DataBatchJobDefinition_EksProperties_PodProperties_Volumes {
            empty_dir: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef {
        DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_EmptyDirRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_HostPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_Volumes_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties_PodProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_secrets: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_containers: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_InitContainers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_process_namespace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes>>,
}

impl DataBatchJobDefinition_EksProperties_PodProperties {
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Containers>>,
    ) -> Self {
        self.containers = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `image_pull_secrets`.\n"]
    pub fn set_image_pull_secrets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecrets>>,
    ) -> Self {
        self.image_pull_secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `init_containers`.\n"]
    pub fn set_init_containers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_InitContainers>>,
    ) -> Self {
        self.init_containers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Metadata>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties_Volumes>>,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties_PodProperties {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties_PodProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties_PodProperties {}

impl BuildDataBatchJobDefinition_EksProperties_PodProperties {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties_PodProperties {
        DataBatchJobDefinition_EksProperties_PodProperties {
            containers: core::default::Default::default(),
            dns_policy: core::default::Default::default(),
            host_network: core::default::Default::default(),
            image_pull_secrets: core::default::Default::default(),
            init_containers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            service_account_name: core::default::Default::default(),
            share_process_namespace: core::default::Default::default(),
            volumes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksProperties_PodPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksProperties_PodPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_EksProperties_PodPropertiesRef {
        DataBatchJobDefinition_EksProperties_PodPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksProperties_PodPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_policy` after provisioning.\n"]
    pub fn dns_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `host_network` after provisioning.\n"]
    pub fn host_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_network", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_secrets` after provisioning.\n"]
    pub fn image_pull_secrets(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_ImagePullSecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_pull_secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `init_containers` after provisioning.\n"]
    pub fn init_containers(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_InitContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.init_containers", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\n"]
    pub fn service_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `share_process_namespace` after provisioning.\n"]
    pub fn share_process_namespace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_process_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodProperties_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_EksProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_properties: std::option::Option<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties>>,
}

impl DataBatchJobDefinition_EksProperties {
    #[doc = "Set the field `pod_properties`.\n"]
    pub fn set_pod_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBatchJobDefinition_EksProperties_PodProperties>>,
    ) -> Self {
        self.pod_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_EksProperties {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_EksProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_EksProperties {}

impl BuildDataBatchJobDefinition_EksProperties {
    pub fn build(self) -> DataBatchJobDefinition_EksProperties {
        DataBatchJobDefinition_EksProperties { pod_properties: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_EksPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_EksPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBatchJobDefinition_EksPropertiesRef {
        DataBatchJobDefinition_EksPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_EksPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pod_properties` after provisioning.\n"]
    pub fn pod_properties(&self) -> terrars::ListRef<DataBatchJobDefinition_EksProperties_PodPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
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

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
    type O =
        terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_gib: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
    #[doc = "Set the field `size_in_gib`.\n"]
    pub fn set_size_in_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_in_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage {
            size_in_gib: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
    pub fn build(
        self,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration {
            platform_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
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

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
    type O =
        terrars
        ::BlockAssignable<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
    pub fn build(
        self,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices {
            container_path: core::default::Default::default(),
            host_path: core::default::Default::default(),
            permissions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mount_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs {
            container_path: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef {
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

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    devices: std
    ::option
    ::Option<
        terrars
        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_process_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_swap: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    swappiness: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs: std
    ::option
    ::Option<
        terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs>,
    >,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
    #[doc = "Set the field `devices`.\n"]
    pub fn set_devices(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Devices,
                        >,
                    >,
    ) -> Self {
        self.devices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `init_process_enabled`.\n"]
    pub fn set_init_process_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.init_process_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_swap`.\n"]
    pub fn set_max_swap(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_swap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_memory_size`.\n"]
    pub fn set_shared_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shared_memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `swappiness`.\n"]
    pub fn set_swappiness(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.swappiness = std::option::Option::Some(v.into());
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
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_Tmpfs,
                        >,
                    >,
    ) -> Self {
        self.tmpfs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters {
            devices: core::default::Default::default(),
            init_process_enabled: core::default::Default::default(),
            max_swap: core::default::Default::default(),
            shared_memory_size: core::default::Default::default(),
            swappiness: core::default::Default::default(),
            tmpfs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `devices` after provisioning.\n"]
    pub fn devices(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_DevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.devices", self.base))
    }

    #[doc = "Get a reference to the value of field `init_process_enabled` after provisioning.\n"]
    pub fn init_process_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init_process_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_swap` after provisioning.\n"]
    pub fn max_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_swap", self.base))
    }

    #[doc = "Get a reference to the value of field `shared_memory_size` after provisioning.\n"]
    pub fn shared_memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `swappiness` after provisioning.\n"]
    pub fn swappiness(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.swappiness", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs` after provisioning.\n"]
    pub fn tmpfs(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters_TmpfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_from: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_from`.\n"]
    pub fn set_value_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_from = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
    type O =
        terrars
        ::BlockAssignable<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
    pub fn build(
        self,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions {
            name: core::default::Default::default(),
            value_from: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_options: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions,
        >,
    >,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
    #[doc = "Set the field `log_driver`.\n"]
    pub fn set_log_driver(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_options`.\n"]
    pub fn set_secret_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptions,
                        >,
                    >,
    ) -> Self {
        self.secret_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration {
            log_driver: core::default::Default::default(),
            options: core::default::Default::default(),
            secret_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef {
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

    #[doc = "Get a reference to the value of field `secret_options` after provisioning.\n"]
    pub fn secret_options(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration_SecretOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_volume: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
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

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
    type O =
        terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints {
            container_path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_volume: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.assign_public_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
    #[doc = "Set the field `cpu_architecture`.\n"]
    pub fn set_cpu_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system_family`.\n"]
    pub fn set_operating_system_family(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operating_system_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
    type O =
        terrars
        ::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_architecture` after provisioning.\n"]
    pub fn cpu_architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system_family` after provisioning.\n"]
    pub fn operating_system_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system_family", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_from: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_from`.\n"]
    pub fn set_value_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_from = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets {
            name: core::default::Default::default(),
            value_from: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef {
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
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hard_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    soft_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
    #[doc = "Set the field `hard_limit`.\n"]
    pub fn set_hard_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hard_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `soft_limit`.\n"]
    pub fn set_soft_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.soft_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits {
            hard_limit: core::default::Default::default(),
            name: core::default::Default::default(),
            soft_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef {
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

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
    #[doc = "Set the field `access_point_id`.\n"]
    pub fn set_access_point_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
    pub fn build(
        self,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfig,
                        >,
                    >,
    ) -> Self {
        self.authorization_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption`.\n"]
    pub fn set_transit_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transit_encryption_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
    pub fn build(
        self,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration {
            authorization_config: core::default::Default::default(),
            file_system_id: core::default::Default::default(),
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration_AuthorizationConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption` after provisioning.\n"]
    pub fn transit_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
    #[doc = "Set the field `source_path`.\n"]
    pub fn set_source_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
    type O =
        terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host {
            source_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_path` after provisioning.\n"]
    pub fn source_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_volume_configuration: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfiguration,
                        >,
                    >,
    ) -> Self {
        self.efs_volume_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_Host>,
                    >,
    ) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes {
            efs_volume_configuration: core::default::Default::default(),
            host: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_EfsVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes_HostRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std
    ::option
    ::Option<
        terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fargate_platform_configuration: std
    ::option
    ::Option<
        terrars
        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_parameters: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std
    ::option
    ::Option<
        terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_points: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<
        terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readonly_root_filesystem: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_requirements: std
    ::option
    ::Option<
        terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_platform: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ulimits: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Environment>,
                    >,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorage,
                        >,
                    >,
    ) -> Self {
        self.ephemeral_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fargate_platform_configuration`.\n"]
    pub fn set_fargate_platform_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfiguration,
                        >,
                    >,
    ) -> Self {
        self.fargate_platform_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_role_arn`.\n"]
    pub fn set_job_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linux_parameters`.\n"]
    pub fn set_linux_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParameters,
                        >,
                    >,
    ) -> Self {
        self.linux_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfiguration,
                        >,
                    >,
    ) -> Self {
        self.log_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_points`.\n"]
    pub fn set_mount_points(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPoints>,
                    >,
    ) -> Self {
        self.mount_points = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfiguration,
                        >,
                    >,
    ) -> Self {
        self.network_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.privileged = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `readonly_root_filesystem`.\n"]
    pub fn set_readonly_root_filesystem(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.readonly_root_filesystem = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_requirements`.\n"]
    pub fn set_resource_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirements,
                        >,
                    >,
    ) -> Self {
        self.resource_requirements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_platform`.\n"]
    pub fn set_runtime_platform(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatform,
                        >,
                    >,
    ) -> Self {
        self.runtime_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets`.\n"]
    pub fn set_secrets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Secrets>,
                    >,
    ) -> Self {
        self.secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ulimits`.\n"]
    pub fn set_ulimits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Ulimits>,
                    >,
    ) -> Self {
        self.ulimits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_Volumes>,
                    >,
    ) -> Self {
        self.volumes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container {
            command: core::default::Default::default(),
            environment: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            fargate_platform_configuration: core::default::Default::default(),
            image: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            job_role_arn: core::default::Default::default(),
            linux_parameters: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            mount_points: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            privileged: core::default::Default::default(),
            readonly_root_filesystem: core::default::Default::default(),
            resource_requirements: core::default::Default::default(),
            runtime_platform: core::default::Default::default(),
            secrets: core::default::Default::default(),
            ulimits: core::default::Default::default(),
            user: core::default::Default::default(),
            volumes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `fargate_platform_configuration` after provisioning.\n"]
    pub fn fargate_platform_configuration(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_FargatePlatformConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fargate_platform_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `job_role_arn` after provisioning.\n"]
    pub fn job_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `linux_parameters` after provisioning.\n"]
    pub fn linux_parameters(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LinuxParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_points` after provisioning.\n"]
    pub fn mount_points(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_MountPointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_points", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }

    #[doc = "Get a reference to the value of field `readonly_root_filesystem` after provisioning.\n"]
    pub fn readonly_root_filesystem(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readonly_root_filesystem", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_requirements` after provisioning.\n"]
    pub fn resource_requirements(
        &self,
    ) -> terrars
    ::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_ResourceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_RuntimePlatformRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_SecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `ulimits` after provisioning.\n"]
    pub fn ulimits(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_UlimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ulimits", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangeProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_nodes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_NodeProperties_NodeRangeProperties {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_Container>>,
    ) -> Self {
        self.container = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_nodes`.\n"]
    pub fn set_target_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties_NodeRangeProperties {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties_NodeRangeProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties {}

impl BuildDataBatchJobDefinition_NodeProperties_NodeRangeProperties {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties_NodeRangeProperties {
        DataBatchJobDefinition_NodeProperties_NodeRangeProperties {
            container: core::default::Default::default(),
            target_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef {
        DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangeProperties_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc = "Get a reference to the value of field `target_nodes` after provisioning.\n"]
    pub fn target_nodes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_NodeProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_range_properties: std
    ::option
    ::Option<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_nodes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_NodeProperties {
    #[doc = "Set the field `main_node`.\n"]
    pub fn set_main_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.main_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_range_properties`.\n"]
    pub fn set_node_range_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBatchJobDefinition_NodeProperties_NodeRangeProperties>>,
    ) -> Self {
        self.node_range_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_nodes`.\n"]
    pub fn set_num_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_NodeProperties {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_NodeProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_NodeProperties {}

impl BuildDataBatchJobDefinition_NodeProperties {
    pub fn build(self) -> DataBatchJobDefinition_NodeProperties {
        DataBatchJobDefinition_NodeProperties {
            main_node: core::default::Default::default(),
            node_range_properties: core::default::Default::default(),
            num_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_NodePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_NodePropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBatchJobDefinition_NodePropertiesRef {
        DataBatchJobDefinition_NodePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_NodePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `main_node` after provisioning.\n"]
    pub fn main_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_node", self.base))
    }

    #[doc = "Get a reference to the value of field `node_range_properties` after provisioning.\n"]
    pub fn node_range_properties(
        &self,
    ) -> terrars::ListRef<DataBatchJobDefinition_NodeProperties_NodeRangePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_range_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `num_nodes` after provisioning.\n"]
    pub fn num_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_exit_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_RetryStrategy_EvaluateOnExit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_RetryStrategy_EvaluateOnExit {}

impl BuildDataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
    pub fn build(self) -> DataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
        DataBatchJobDefinition_RetryStrategy_EvaluateOnExit {
            action: core::default::Default::default(),
            on_exit_code: core::default::Default::default(),
            on_reason: core::default::Default::default(),
            on_status_reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
        DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef {
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

#[derive(Serialize)]
pub struct DataBatchJobDefinition_RetryStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate_on_exit: std::option::Option<terrars::ListField<DataBatchJobDefinition_RetryStrategy_EvaluateOnExit>>,
}

impl DataBatchJobDefinition_RetryStrategy {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBatchJobDefinition_RetryStrategy_EvaluateOnExit>>,
    ) -> Self {
        self.evaluate_on_exit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_RetryStrategy {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_RetryStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_RetryStrategy {}

impl BuildDataBatchJobDefinition_RetryStrategy {
    pub fn build(self) -> DataBatchJobDefinition_RetryStrategy {
        DataBatchJobDefinition_RetryStrategy {
            attempts: core::default::Default::default(),
            evaluate_on_exit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_RetryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_RetryStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBatchJobDefinition_RetryStrategyRef {
        DataBatchJobDefinition_RetryStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_RetryStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(&self) -> terrars::ListRef<DataBatchJobDefinition_RetryStrategy_EvaluateOnExitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate_on_exit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobDefinition_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempt_duration_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchJobDefinition_Timeout {
    #[doc = "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchJobDefinition_Timeout {
    type O = terrars::BlockAssignable<DataBatchJobDefinition_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobDefinition_Timeout {}

impl BuildDataBatchJobDefinition_Timeout {
    pub fn build(self) -> DataBatchJobDefinition_Timeout {
        DataBatchJobDefinition_Timeout { attempt_duration_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBatchJobDefinition_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchJobDefinition_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBatchJobDefinition_TimeoutRef {
        DataBatchJobDefinition_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobDefinition_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_duration_seconds", self.base))
    }
}
