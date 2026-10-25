#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrserverlessApplicationData_ {
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
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    release_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_start_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_AutoStartConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_stop_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_AutoStopConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_ImageConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_capacity: std::option::Option<std::vec::Vec<EmrserverlessApplication_InitialCapacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interactive_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_InteractiveConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_level_cost_allocation_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_JobLevelCostAllocationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_capacity: std::option::Option<std::vec::Vec<EmrserverlessApplication_MaximumCapacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_RuntimeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplication_SchedulerConfiguration>>,
    dynamic: EmrserverlessApplicationDynamic,
}

struct EmrserverlessApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrserverlessApplicationData_>,
}

#[derive(Clone)]
pub struct EmrserverlessApplication(std::rc::Rc<EmrserverlessApplication_>);

impl EmrserverlessApplication {
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

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().architecture = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `auto_start_configuration`.\n"]
    pub fn set_auto_start_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_AutoStartConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_start_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_start_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_stop_configuration`.\n"]
    pub fn set_auto_stop_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_AutoStopConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_stop_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_stop_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_configuration`.\n"]
    pub fn set_image_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_ImageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_capacity`.\n"]
    pub fn set_initial_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_InitialCapacity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interactive_configuration`.\n"]
    pub fn set_interactive_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_InteractiveConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interactive_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interactive_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_level_cost_allocation_configuration`.\n"]
    pub fn set_job_level_cost_allocation_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrserverlessApplication_JobLevelCostAllocationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_level_cost_allocation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_level_cost_allocation_configuration =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maximum_capacity`.\n"]
    pub fn set_maximum_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_MaximumCapacity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maximum_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maximum_capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_configuration`.\n"]
    pub fn set_runtime_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_RuntimeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduler_configuration`.\n"]
    pub fn set_scheduler_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplication_SchedulerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_AutoStartConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_AutoStopConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_ImageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interactive_configuration` after provisioning.\n"]
    pub fn interactive_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_InteractiveConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interactive_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_level_cost_allocation_configuration` after provisioning.\n"]
    pub fn job_level_cost_allocation_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_JobLevelCostAllocationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.job_level_cost_allocation_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> terrars::ListRef<EmrserverlessApplication_MaximumCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_MonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_RuntimeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduler_configuration` after provisioning.\n"]
    pub fn scheduler_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_SchedulerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for EmrserverlessApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrserverlessApplication { }

impl terrars::ToListMappable for EmrserverlessApplication {
    type O = terrars::ListRef<EmrserverlessApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrserverlessApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emrserverless_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrserverlessApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrserverlessApplication {
        let out = EmrserverlessApplication(std::rc::Rc::new(EmrserverlessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrserverlessApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                architecture: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                release_label: self.release_label,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                auto_start_configuration: core::default::Default::default(),
                auto_stop_configuration: core::default::Default::default(),
                image_configuration: core::default::Default::default(),
                initial_capacity: core::default::Default::default(),
                interactive_configuration: core::default::Default::default(),
                job_level_cost_allocation_configuration: core::default::Default::default(),
                maximum_capacity: core::default::Default::default(),
                monitoring_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                runtime_configuration: core::default::Default::default(),
                scheduler_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrserverlessApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrserverlessApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_AutoStartConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_AutoStopConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_ImageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interactive_configuration` after provisioning.\n"]
    pub fn interactive_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_InteractiveConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interactive_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_level_cost_allocation_configuration` after provisioning.\n"]
    pub fn job_level_cost_allocation_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_JobLevelCostAllocationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.job_level_cost_allocation_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> terrars::ListRef<EmrserverlessApplication_MaximumCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_MonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_RuntimeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduler_configuration` after provisioning.\n"]
    pub fn scheduler_configuration(&self) -> terrars::ListRef<EmrserverlessApplication_SchedulerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationDynamic {
    auto_start_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_AutoStartConfiguration>>,
    auto_stop_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_AutoStopConfiguration>>,
    image_configuration: std::option::Option<terrars::DynamicBlock<EmrserverlessApplication_ImageConfiguration>>,
    initial_capacity: std::option::Option<terrars::DynamicBlock<EmrserverlessApplication_InitialCapacity>>,
    interactive_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_InteractiveConfiguration>>,
    job_level_cost_allocation_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_JobLevelCostAllocationConfiguration>>,
    maximum_capacity: std::option::Option<terrars::DynamicBlock<EmrserverlessApplication_MaximumCapacity>>,
    monitoring_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<EmrserverlessApplication_NetworkConfiguration>>,
    runtime_configuration: std::option::Option<terrars::DynamicBlock<EmrserverlessApplication_RuntimeConfiguration>>,
    scheduler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_SchedulerConfiguration>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_AutoStartConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EmrserverlessApplication_AutoStartConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_AutoStartConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_AutoStartConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_AutoStartConfiguration {}

impl BuildEmrserverlessApplication_AutoStartConfiguration {
    pub fn build(self) -> EmrserverlessApplication_AutoStartConfiguration {
        EmrserverlessApplication_AutoStartConfiguration { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_AutoStartConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_AutoStartConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_AutoStartConfigurationRef {
        EmrserverlessApplication_AutoStartConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_AutoStartConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_AutoStopConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrserverlessApplication_AutoStopConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout_minutes`.\n"]
    pub fn set_idle_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_AutoStopConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_AutoStopConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_AutoStopConfiguration {}

impl BuildEmrserverlessApplication_AutoStopConfiguration {
    pub fn build(self) -> EmrserverlessApplication_AutoStopConfiguration {
        EmrserverlessApplication_AutoStopConfiguration {
            enabled: core::default::Default::default(),
            idle_timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_AutoStopConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_AutoStopConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_AutoStopConfigurationRef {
        EmrserverlessApplication_AutoStopConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_AutoStopConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `idle_timeout_minutes` after provisioning.\n"]
    pub fn idle_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_ImageConfiguration {
    image_uri: terrars::PrimField<std::string::String>,
}

impl EmrserverlessApplication_ImageConfiguration { }

impl terrars::ToListMappable for EmrserverlessApplication_ImageConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_ImageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_ImageConfiguration {
    #[doc = ""]
    pub image_uri: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication_ImageConfiguration {
    pub fn build(self) -> EmrserverlessApplication_ImageConfiguration {
        EmrserverlessApplication_ImageConfiguration { image_uri: self.image_uri }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_ImageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_ImageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_ImageConfigurationRef {
        EmrserverlessApplication_ImageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_ImageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
    cpu: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<std::string::String>>,
    memory: terrars::PrimField<std::string::String>,
}

impl EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
    type O =
        terrars::BlockAssignable<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
    #[doc = ""]
    pub cpu: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub memory: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
    pub fn build(self) -> EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
        EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef {
        EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplication_InitialCapacity_InitialCapacityConfigDynamic {
    worker_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
    worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration>>,
    dynamic: EmrserverlessApplication_InitialCapacity_InitialCapacityConfigDynamic,
}

impl EmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
    type O = terrars::BlockAssignable<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
    #[doc = ""]
    pub worker_count: terrars::PrimField<f64>,
}

impl BuildEmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
    pub fn build(self) -> EmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
        EmrserverlessApplication_InitialCapacity_InitialCapacityConfig {
            worker_count: self.worker_count,
            worker_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef {
        EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig_WorkerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplication_InitialCapacityDynamic {
    initial_capacity_config: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_InitialCapacity {
    initial_capacity_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_capacity_config: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig>>,
    dynamic: EmrserverlessApplication_InitialCapacityDynamic,
}

impl EmrserverlessApplication_InitialCapacity {
    #[doc = "Set the field `initial_capacity_config`.\n"]
    pub fn set_initial_capacity_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrserverlessApplication_InitialCapacity_InitialCapacityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initial_capacity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_capacity_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_InitialCapacity {
    type O = terrars::BlockAssignable<EmrserverlessApplication_InitialCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_InitialCapacity {
    #[doc = ""]
    pub initial_capacity_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication_InitialCapacity {
    pub fn build(self) -> EmrserverlessApplication_InitialCapacity {
        EmrserverlessApplication_InitialCapacity {
            initial_capacity_type: self.initial_capacity_type,
            initial_capacity_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_InitialCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_InitialCapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrserverlessApplication_InitialCapacityRef {
        EmrserverlessApplication_InitialCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_InitialCapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_capacity_type` after provisioning.\n"]
    pub fn initial_capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_capacity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_capacity_config` after provisioning.\n"]
    pub fn initial_capacity_config(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_InitialCapacity_InitialCapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_capacity_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_InteractiveConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livy_endpoint_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    studio_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EmrserverlessApplication_InteractiveConfiguration {
    #[doc = "Set the field `livy_endpoint_enabled`.\n"]
    pub fn set_livy_endpoint_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.livy_endpoint_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `studio_enabled`.\n"]
    pub fn set_studio_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.studio_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_InteractiveConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_InteractiveConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_InteractiveConfiguration {}

impl BuildEmrserverlessApplication_InteractiveConfiguration {
    pub fn build(self) -> EmrserverlessApplication_InteractiveConfiguration {
        EmrserverlessApplication_InteractiveConfiguration {
            livy_endpoint_enabled: core::default::Default::default(),
            studio_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_InteractiveConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_InteractiveConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_InteractiveConfigurationRef {
        EmrserverlessApplication_InteractiveConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_InteractiveConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `livy_endpoint_enabled` after provisioning.\n"]
    pub fn livy_endpoint_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livy_endpoint_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_enabled` after provisioning.\n"]
    pub fn studio_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.studio_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_JobLevelCostAllocationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EmrserverlessApplication_JobLevelCostAllocationConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_JobLevelCostAllocationConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_JobLevelCostAllocationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_JobLevelCostAllocationConfiguration {}

impl BuildEmrserverlessApplication_JobLevelCostAllocationConfiguration {
    pub fn build(self) -> EmrserverlessApplication_JobLevelCostAllocationConfiguration {
        EmrserverlessApplication_JobLevelCostAllocationConfiguration { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_JobLevelCostAllocationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_JobLevelCostAllocationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_JobLevelCostAllocationConfigurationRef {
        EmrserverlessApplication_JobLevelCostAllocationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_JobLevelCostAllocationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MaximumCapacity {
    cpu: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<std::string::String>>,
    memory: terrars::PrimField<std::string::String>,
}

impl EmrserverlessApplication_MaximumCapacity {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_MaximumCapacity {
    type O = terrars::BlockAssignable<EmrserverlessApplication_MaximumCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MaximumCapacity {
    #[doc = ""]
    pub cpu: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub memory: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication_MaximumCapacity {
    pub fn build(self) -> EmrserverlessApplication_MaximumCapacity {
        EmrserverlessApplication_MaximumCapacity {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MaximumCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MaximumCapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrserverlessApplication_MaximumCapacityRef {
        EmrserverlessApplication_MaximumCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MaximumCapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes { }

impl terrars
::ToListMappable for EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
    type O =
        terrars
        ::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
        EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypesRef {
        EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationDynamic {
    log_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_types: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes>>,
    dynamic: EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationDynamic,
}

impl EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream_name_prefix`.\n"]
    pub fn set_log_stream_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_stream_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_types`.\n"]
    pub fn set_log_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration_LogTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
    type O =
        terrars::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
        EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration {
            enabled: self.enabled,
            encryption_key_arn: core::default::Default::default(),
            log_group_name: core::default::Default::default(),
            log_stream_name_prefix: core::default::Default::default(),
            log_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef {
        EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name_prefix` after provisioning.\n"]
    pub fn log_stream_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
    type O =
        terrars
        ::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {}

impl BuildEmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
        EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration {
            enabled: core::default::Default::default(),
            encryption_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef {
        EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_write_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
    #[doc = "Set the field `remote_write_url`.\n"]
    pub fn set_remote_write_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.remote_write_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
    type O =
        terrars::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {}

impl BuildEmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
        EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration {
            remote_write_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef {
        EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_write_url` after provisioning.\n"]
    pub fn remote_write_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_write_url", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
    #[doc = "Set the field `encryption_key_arn`.\n"]
    pub fn set_encryption_key_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_uri`.\n"]
    pub fn set_log_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {}

impl BuildEmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
        EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration {
            encryption_key_arn: core::default::Default::default(),
            log_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef {
        EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_key_arn` after provisioning.\n"]
    pub fn encryption_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplication_MonitoringConfigurationDynamic {
    cloudwatch_logging_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration>>,
    managed_persistence_monitoring_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration>,
    >,
    prometheus_monitoring_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration>,
    >,
    s3_monitoring_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_MonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_persistence_monitoring_configuration: std
    ::option
    ::Option<
        std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prometheus_monitoring_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_monitoring_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration>>,
    dynamic: EmrserverlessApplication_MonitoringConfigurationDynamic,
}

impl EmrserverlessApplication_MonitoringConfiguration {
    #[doc = "Set the field `cloudwatch_logging_configuration`.\n"]
    pub fn set_cloudwatch_logging_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_persistence_monitoring_configuration`.\n"]
    pub fn set_managed_persistence_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_persistence_monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_persistence_monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prometheus_monitoring_configuration`.\n"]
    pub fn set_prometheus_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prometheus_monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prometheus_monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_monitoring_configuration`.\n"]
    pub fn set_s3_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_MonitoringConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_MonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_MonitoringConfiguration {}

impl BuildEmrserverlessApplication_MonitoringConfiguration {
    pub fn build(self) -> EmrserverlessApplication_MonitoringConfiguration {
        EmrserverlessApplication_MonitoringConfiguration {
            cloudwatch_logging_configuration: core::default::Default::default(),
            managed_persistence_monitoring_configuration: core::default::Default::default(),
            prometheus_monitoring_configuration: core::default::Default::default(),
            s3_monitoring_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_MonitoringConfigurationRef {
        EmrserverlessApplication_MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_configuration` after provisioning.\n"]
    pub fn cloudwatch_logging_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_MonitoringConfiguration_CloudwatchLoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_persistence_monitoring_configuration` after provisioning.\n"]
    pub fn managed_persistence_monitoring_configuration(
        &self,
    ) -> terrars
    ::ListRef<EmrserverlessApplication_MonitoringConfiguration_ManagedPersistenceMonitoringConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.managed_persistence_monitoring_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `prometheus_monitoring_configuration` after provisioning.\n"]
    pub fn prometheus_monitoring_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_MonitoringConfiguration_PrometheusMonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prometheus_monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_monitoring_configuration` after provisioning.\n"]
    pub fn s3_monitoring_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplication_MonitoringConfiguration_S3MonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_monitoring_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EmrserverlessApplication_NetworkConfiguration {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_NetworkConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_NetworkConfiguration {}

impl BuildEmrserverlessApplication_NetworkConfiguration {
    pub fn build(self) -> EmrserverlessApplication_NetworkConfiguration {
        EmrserverlessApplication_NetworkConfiguration {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_NetworkConfigurationRef {
        EmrserverlessApplication_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_RuntimeConfiguration {
    classification: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EmrserverlessApplication_RuntimeConfiguration {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_RuntimeConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_RuntimeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_RuntimeConfiguration {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication_RuntimeConfiguration {
    pub fn build(self) -> EmrserverlessApplication_RuntimeConfiguration {
        EmrserverlessApplication_RuntimeConfiguration {
            classification: self.classification,
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_RuntimeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_RuntimeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_RuntimeConfigurationRef {
        EmrserverlessApplication_RuntimeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_RuntimeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplication_SchedulerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_runs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrserverlessApplication_SchedulerConfiguration {
    #[doc = "Set the field `max_concurrent_runs`.\n"]
    pub fn set_max_concurrent_runs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_runs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_timeout_minutes`.\n"]
    pub fn set_queue_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.queue_timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplication_SchedulerConfiguration {
    type O = terrars::BlockAssignable<EmrserverlessApplication_SchedulerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplication_SchedulerConfiguration {}

impl BuildEmrserverlessApplication_SchedulerConfiguration {
    pub fn build(self) -> EmrserverlessApplication_SchedulerConfiguration {
        EmrserverlessApplication_SchedulerConfiguration {
            max_concurrent_runs: core::default::Default::default(),
            queue_timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrserverlessApplication_SchedulerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplication_SchedulerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplication_SchedulerConfigurationRef {
        EmrserverlessApplication_SchedulerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplication_SchedulerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_runs` after provisioning.\n"]
    pub fn max_concurrent_runs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_runs", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_timeout_minutes` after provisioning.\n"]
    pub fn queue_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_timeout_minutes", self.base))
    }
}
