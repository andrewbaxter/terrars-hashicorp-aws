use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BatchComputeEnvironmentData {
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
    compute_environment_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_environment_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_resources: std::option::Option<std::vec::Vec<BatchComputeEnvironmentComputeResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_configuration: std::option::Option<std::vec::Vec<BatchComputeEnvironmentEksConfigurationEl>>,
    dynamic: BatchComputeEnvironmentDynamic,
}

struct BatchComputeEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BatchComputeEnvironmentData>,
}

#[derive(Clone)]
pub struct BatchComputeEnvironment(std::rc::Rc<BatchComputeEnvironment_>);

impl BatchComputeEnvironment {
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

    #[doc = "Set the field `compute_environment_name`.\n"]
    pub fn set_compute_environment_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().compute_environment_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_environment_name_prefix`.\n"]
    pub fn set_compute_environment_name_prefix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().compute_environment_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_role`.\n"]
    pub fn set_service_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `compute_resources`.\n"]
    pub fn set_compute_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eks_configuration`.\n"]
    pub fn set_eks_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchComputeEnvironmentEksConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eks_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eks_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_environment_name` after provisioning.\n"]
    pub fn compute_environment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_environment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_environment_name_prefix` after provisioning.\n"]
    pub fn compute_environment_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_environment_name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_resources` after provisioning.\n"]
    pub fn compute_resources(&self) -> terrars::ListRef<BatchComputeEnvironmentComputeResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_configuration` after provisioning.\n"]
    pub fn eks_configuration(&self) -> terrars::ListRef<BatchComputeEnvironmentEksConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for BatchComputeEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BatchComputeEnvironment { }

impl terrars::ToListMappable for BatchComputeEnvironment {
    type O = terrars::ListRef<BatchComputeEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BatchComputeEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_batch_compute_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchComputeEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBatchComputeEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> BatchComputeEnvironment {
        let out = BatchComputeEnvironment(std::rc::Rc::new(BatchComputeEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BatchComputeEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                compute_environment_name: core::default::Default::default(),
                compute_environment_name_prefix: core::default::Default::default(),
                id: core::default::Default::default(),
                service_role: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                compute_resources: core::default::Default::default(),
                eks_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchComputeEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchComputeEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BatchComputeEnvironmentRef {
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

    #[doc = "Get a reference to the value of field `compute_environment_name` after provisioning.\n"]
    pub fn compute_environment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_environment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_environment_name_prefix` after provisioning.\n"]
    pub fn compute_environment_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_environment_name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_resources` after provisioning.\n"]
    pub fn compute_resources(&self) -> terrars::ListRef<BatchComputeEnvironmentComputeResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eks_configuration` after provisioning.\n"]
    pub fn eks_configuration(&self) -> terrars::ListRef<BatchComputeEnvironmentEksConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_id_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    #[doc = "Set the field `image_id_override`.\n"]
    pub fn set_image_id_override(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_id_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    type O = terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {}

impl BuildBatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
        BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
            image_id_override: core::default::Default::default(),
            image_type: core::default::Default::default(),
        }
    }
}

pub struct BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
        BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_id_override` after provisioning.\n"]
    pub fn image_id_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id_override", self.base))
    }

    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    type O = terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {}

impl BuildBatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
        BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
        BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchComputeEnvironmentComputeResourcesElDynamic {
    ec2_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    launch_template: std
    ::option
    ::Option<terrars::DynamicBlock<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
}

#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_vcpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_key_pair: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    max_vcpus: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_vcpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_iam_fleet_role: std::option::Option<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_configuration: std::option::Option<std::vec::Vec<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
    dynamic: BatchComputeEnvironmentComputeResourcesElDynamic,
}

impl BatchComputeEnvironmentComputeResourcesEl {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocation_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bid_percentage`.\n"]
    pub fn set_bid_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bid_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_vcpus`.\n"]
    pub fn set_desired_vcpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_vcpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ec2_key_pair`.\n"]
    pub fn set_ec2_key_pair(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ec2_key_pair = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_vcpus`.\n"]
    pub fn set_min_vcpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_vcpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_iam_fleet_role`.\n"]
    pub fn set_spot_iam_fleet_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spot_iam_fleet_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ec2_configuration`.\n"]
    pub fn set_ec2_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchComputeEnvironmentComputeResourcesEl {
    type O = terrars::BlockAssignable<BatchComputeEnvironmentComputeResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchComputeEnvironmentComputeResourcesEl {
    #[doc = ""]
    pub max_vcpus: terrars::PrimField<f64>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBatchComputeEnvironmentComputeResourcesEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesEl {
        BatchComputeEnvironmentComputeResourcesEl {
            allocation_strategy: core::default::Default::default(),
            bid_percentage: core::default::Default::default(),
            desired_vcpus: core::default::Default::default(),
            ec2_key_pair: core::default::Default::default(),
            image_id: core::default::Default::default(),
            instance_role: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            max_vcpus: self.max_vcpus,
            min_vcpus: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            spot_iam_fleet_role: core::default::Default::default(),
            subnets: self.subnets,
            tags: core::default::Default::default(),
            type_: self.type_,
            ec2_configuration: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BatchComputeEnvironmentComputeResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchComputeEnvironmentComputeResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchComputeEnvironmentComputeResourcesElRef {
        BatchComputeEnvironmentComputeResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchComputeEnvironmentComputeResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_percentage` after provisioning.\n"]
    pub fn bid_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bid_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_vcpus` after provisioning.\n"]
    pub fn desired_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_vcpus", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2_key_pair` after provisioning.\n"]
    pub fn ec2_key_pair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_key_pair", self.base))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `max_vcpus` after provisioning.\n"]
    pub fn max_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_vcpus", self.base))
    }

    #[doc = "Get a reference to the value of field `min_vcpus` after provisioning.\n"]
    pub fn min_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_vcpus", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_iam_fleet_role` after provisioning.\n"]
    pub fn spot_iam_fleet_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_iam_fleet_role", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2_configuration` after provisioning.\n"]
    pub fn ec2_configuration(&self) -> terrars::ListRef<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchComputeEnvironmentEksConfigurationEl {
    eks_cluster_arn: terrars::PrimField<std::string::String>,
    kubernetes_namespace: terrars::PrimField<std::string::String>,
}

impl BatchComputeEnvironmentEksConfigurationEl { }

impl terrars::ToListMappable for BatchComputeEnvironmentEksConfigurationEl {
    type O = terrars::BlockAssignable<BatchComputeEnvironmentEksConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchComputeEnvironmentEksConfigurationEl {
    #[doc = ""]
    pub eks_cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub kubernetes_namespace: terrars::PrimField<std::string::String>,
}

impl BuildBatchComputeEnvironmentEksConfigurationEl {
    pub fn build(self) -> BatchComputeEnvironmentEksConfigurationEl {
        BatchComputeEnvironmentEksConfigurationEl {
            eks_cluster_arn: self.eks_cluster_arn,
            kubernetes_namespace: self.kubernetes_namespace,
        }
    }
}

pub struct BatchComputeEnvironmentEksConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchComputeEnvironmentEksConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchComputeEnvironmentEksConfigurationElRef {
        BatchComputeEnvironmentEksConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchComputeEnvironmentEksConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks_cluster_arn` after provisioning.\n"]
    pub fn eks_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eks_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes_namespace` after provisioning.\n"]
    pub fn kubernetes_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchComputeEnvironmentDynamic {
    compute_resources: std::option::Option<terrars::DynamicBlock<BatchComputeEnvironmentComputeResourcesEl>>,
    eks_configuration: std::option::Option<terrars::DynamicBlock<BatchComputeEnvironmentEksConfigurationEl>>,
}
