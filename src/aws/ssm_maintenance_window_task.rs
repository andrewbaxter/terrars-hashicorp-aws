#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmMaintenanceWindowTaskData_ {
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
    cutoff_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_errors: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    task_arn: terrars::PrimField<std::string::String>,
    task_type: terrars::PrimField<std::string::String>,
    window_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<std::vec::Vec<SsmMaintenanceWindowTask_Targets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_invocation_parameters: std::option::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters>>,
    dynamic: SsmMaintenanceWindowTaskDynamic,
}

struct SsmMaintenanceWindowTask_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmMaintenanceWindowTaskData_>,
}

#[derive(Clone)]
pub struct SsmMaintenanceWindowTask(std::rc::Rc<SsmMaintenanceWindowTask_>);

impl SsmMaintenanceWindowTask {
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

    #[doc = "Set the field `cutoff_behavior`.\n"]
    pub fn set_cutoff_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cutoff_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency`.\n"]
    pub fn set_max_concurrency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_errors`.\n"]
    pub fn set_max_errors(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmMaintenanceWindowTask_Targets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task_invocation_parameters`.\n"]
    pub fn set_task_invocation_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task_invocation_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task_invocation_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cutoff_behavior` after provisioning.\n"]
    pub fn cutoff_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cutoff_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arn` after provisioning.\n"]
    pub fn task_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_task_id` after provisioning.\n"]
    pub fn window_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<SsmMaintenanceWindowTask_TargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_invocation_parameters` after provisioning.\n"]
    pub fn task_invocation_parameters(&self) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_invocation_parameters", self.extract_ref()))
    }
}

impl terrars::Referable for SsmMaintenanceWindowTask {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmMaintenanceWindowTask { }

impl terrars::ToListMappable for SsmMaintenanceWindowTask {
    type O = terrars::ListRef<SsmMaintenanceWindowTaskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmMaintenanceWindowTask_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssm_maintenance_window_task".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmMaintenanceWindowTask {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub task_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub window_id: terrars::PrimField<std::string::String>,
}

impl BuildSsmMaintenanceWindowTask {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmMaintenanceWindowTask {
        let out = SsmMaintenanceWindowTask(std::rc::Rc::new(SsmMaintenanceWindowTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmMaintenanceWindowTaskData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cutoff_behavior: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                max_concurrency: core::default::Default::default(),
                max_errors: core::default::Default::default(),
                name: core::default::Default::default(),
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                service_role_arn: core::default::Default::default(),
                task_arn: self.task_arn,
                task_type: self.task_type,
                window_id: self.window_id,
                targets: core::default::Default::default(),
                task_invocation_parameters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmMaintenanceWindowTaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmMaintenanceWindowTaskRef {
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

    #[doc = "Get a reference to the value of field `cutoff_behavior` after provisioning.\n"]
    pub fn cutoff_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cutoff_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arn` after provisioning.\n"]
    pub fn task_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_id` after provisioning.\n"]
    pub fn window_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window_task_id` after provisioning.\n"]
    pub fn window_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<SsmMaintenanceWindowTask_TargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_invocation_parameters` after provisioning.\n"]
    pub fn task_invocation_parameters(&self) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_invocation_parameters", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTaskDynamic {
    targets: std::option::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_Targets>>,
    task_invocation_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters>>,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_Targets {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmMaintenanceWindowTask_Targets { }

impl terrars::ToListMappable for SsmMaintenanceWindowTask_Targets {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_Targets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_Targets {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmMaintenanceWindowTask_Targets {
    pub fn build(self) -> SsmMaintenanceWindowTask_Targets {
        SsmMaintenanceWindowTask_Targets {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmMaintenanceWindowTask_TargetsRef {
        SsmMaintenanceWindowTask_TargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter { }

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
    type O =
        terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
        SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_ParameterRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersDynamic {
    parameter: std
    ::option
    ::Option<
        terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter>,
    >,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter>>,
    dynamic: SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersDynamic,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
        SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters {
            document_version: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
    #[doc = "Set the field `client_context`.\n"]
    pub fn set_client_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\n"]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
        SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters {
            client_context: core::default::Default::default(),
            payload: core::default::Default::default(),
            qualifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_context` after provisioning.\n"]
    pub fn client_context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_context", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_output_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
    #[doc = "Set the field `cloudwatch_log_group_name`.\n"]
    pub fn set_cloudwatch_log_group_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudwatch_log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_output_enabled`.\n"]
    pub fn set_cloudwatch_output_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudwatch_output_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
    type O =
        terrars
        ::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig {
            cloudwatch_log_group_name: core::default::Default::default(),
            cloudwatch_output_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_name` after provisioning.\n"]
    pub fn cloudwatch_log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_output_enabled` after provisioning.\n"]
    pub fn cloudwatch_output_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_output_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
    #[doc = "Set the field `notification_arn`.\n"]
    pub fn set_notification_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.notification_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_events`.\n"]
    pub fn set_notification_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.notification_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_type`.\n"]
    pub fn set_notification_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.notification_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
    type O =
        terrars
        ::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig {
            notification_arn: core::default::Default::default(),
            notification_events: core::default::Default::default(),
            notification_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notification_arn` after provisioning.\n"]
    pub fn notification_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_events` after provisioning.\n"]
    pub fn notification_events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_events", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersDynamic {
    cloudwatch_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig>,
    >,
    notification_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig>,
    >,
    parameter: std
    ::option
    ::Option<
        terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter>,
    >,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_hash_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_config: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std
    ::option
    ::Option<
        std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter>>,
    dynamic: SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersDynamic,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_hash`.\n"]
    pub fn set_document_hash(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_hash = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_hash_type`.\n"]
    pub fn set_document_hash_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.document_hash_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_s3_bucket`.\n"]
    pub fn set_output_s3_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_s3_key_prefix`.\n"]
    pub fn set_output_s3_key_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_s3_key_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_role_arn`.\n"]
    pub fn set_service_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_config`.\n"]
    pub fn set_cloudwatch_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters_Parameter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters {
            comment: core::default::Default::default(),
            document_hash: core::default::Default::default(),
            document_hash_type: core::default::Default::default(),
            document_version: core::default::Default::default(),
            output_s3_bucket: core::default::Default::default(),
            output_s3_key_prefix: core::default::Default::default(),
            service_role_arn: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            cloudwatch_config: core::default::Default::default(),
            notification_config: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `document_hash` after provisioning.\n"]
    pub fn document_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_hash", self.base))
    }

    #[doc = "Get a reference to the value of field `document_hash_type` after provisioning.\n"]
    pub fn document_hash_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_hash_type", self.base))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }

    #[doc = "Get a reference to the value of field `output_s3_bucket` after provisioning.\n"]
    pub fn output_s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `output_s3_key_prefix` after provisioning.\n"]
    pub fn output_s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_s3_key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_config` after provisioning.\n"]
    pub fn cloudwatch_config(
        &self,
    ) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_CloudwatchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_config", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(
        &self,
    ) -> terrars
    ::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
        SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters {
            input: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef {
        SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmMaintenanceWindowTask_TaskInvocationParametersDynamic {
    automation_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters>>,
    lambda_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters>>,
    run_command_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters>>,
    step_functions_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters>>,
}

#[derive(Serialize)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automation_parameters: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_parameters: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command_parameters: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_functions_parameters: std
    ::option
    ::Option<std::vec::Vec<SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters>>,
    dynamic: SsmMaintenanceWindowTask_TaskInvocationParametersDynamic,
}

impl SsmMaintenanceWindowTask_TaskInvocationParameters {
    #[doc = "Set the field `automation_parameters`.\n"]
    pub fn set_automation_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.automation_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.automation_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_parameters`.\n"]
    pub fn set_lambda_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_command_parameters`.\n"]
    pub fn set_run_command_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.run_command_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.run_command_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_functions_parameters`.\n"]
    pub fn set_step_functions_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step_functions_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step_functions_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmMaintenanceWindowTask_TaskInvocationParameters {
    type O = terrars::BlockAssignable<SsmMaintenanceWindowTask_TaskInvocationParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmMaintenanceWindowTask_TaskInvocationParameters {}

impl BuildSsmMaintenanceWindowTask_TaskInvocationParameters {
    pub fn build(self) -> SsmMaintenanceWindowTask_TaskInvocationParameters {
        SsmMaintenanceWindowTask_TaskInvocationParameters {
            automation_parameters: core::default::Default::default(),
            lambda_parameters: core::default::Default::default(),
            run_command_parameters: core::default::Default::default(),
            step_functions_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmMaintenanceWindowTask_TaskInvocationParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmMaintenanceWindowTask_TaskInvocationParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmMaintenanceWindowTask_TaskInvocationParametersRef {
        SsmMaintenanceWindowTask_TaskInvocationParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmMaintenanceWindowTask_TaskInvocationParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automation_parameters` after provisioning.\n"]
    pub fn automation_parameters(
        &self,
    ) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_AutomationParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automation_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_parameters` after provisioning.\n"]
    pub fn lambda_parameters(
        &self,
    ) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_LambdaParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `run_command_parameters` after provisioning.\n"]
    pub fn run_command_parameters(
        &self,
    ) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_RunCommandParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_command_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `step_functions_parameters` after provisioning.\n"]
    pub fn step_functions_parameters(
        &self,
    ) -> terrars::ListRef<SsmMaintenanceWindowTask_TaskInvocationParameters_StepFunctionsParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step_functions_parameters", self.base))
    }
}
