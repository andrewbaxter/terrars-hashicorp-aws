#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudformationStackSetInstanceData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    call_as: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_stack: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_set_instance_region: std::option::Option<terrars::PrimField<std::string::String>>,
    stack_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_targets: std::option::Option<std::vec::Vec<CloudformationStackSetInstance_DeploymentTargets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_preferences: std::option::Option<std::vec::Vec<CloudformationStackSetInstance_OperationPreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudformationStackSetInstance_Timeouts>,
    dynamic: CloudformationStackSetInstanceDynamic,
}

struct CloudformationStackSetInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudformationStackSetInstanceData_>,
}

#[derive(Clone)]
pub struct CloudformationStackSetInstance(std::rc::Rc<CloudformationStackSetInstance_>);

impl CloudformationStackSetInstance {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `call_as`.\n"]
    pub fn set_call_as(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().call_as = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_overrides`.\n"]
    pub fn set_parameter_overrides(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameter_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retain_stack`.\n"]
    pub fn set_retain_stack(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_stack = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack_set_instance_region`.\n"]
    pub fn set_stack_set_instance_region(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().stack_set_instance_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_targets`.\n"]
    pub fn set_deployment_targets(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudformationStackSetInstance_DeploymentTargets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation_preferences`.\n"]
    pub fn set_operation_preferences(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudformationStackSetInstance_OperationPreferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudformationStackSetInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameter_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_stack` after provisioning.\n"]
    pub fn retain_stack(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_stack", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(&self) -> terrars::ListRef<CloudformationStackSetInstance_StackInstanceSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stack_instance_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_instance_region` after provisioning.\n"]
    pub fn stack_set_instance_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_instance_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(&self) -> terrars::ListRef<CloudformationStackSetInstance_DeploymentTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> terrars::ListRef<CloudformationStackSetInstance_OperationPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetInstance_TimeoutsRef {
        CloudformationStackSetInstance_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudformationStackSetInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudformationStackSetInstance { }

impl terrars::ToListMappable for CloudformationStackSetInstance {
    type O = terrars::ListRef<CloudformationStackSetInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudformationStackSetInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudformation_stack_set_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudformationStackSetInstance {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub stack_set_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudformationStackSetInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudformationStackSetInstance {
        let out = CloudformationStackSetInstance(std::rc::Rc::new(CloudformationStackSetInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudformationStackSetInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                call_as: core::default::Default::default(),
                id: core::default::Default::default(),
                parameter_overrides: core::default::Default::default(),
                region: core::default::Default::default(),
                retain_stack: core::default::Default::default(),
                stack_set_instance_region: core::default::Default::default(),
                stack_set_name: self.stack_set_name,
                deployment_targets: core::default::Default::default(),
                operation_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudformationStackSetInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudformationStackSetInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_overrides` after provisioning.\n"]
    pub fn parameter_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameter_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_stack` after provisioning.\n"]
    pub fn retain_stack(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_stack", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stack_instance_summaries` after provisioning.\nList of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set."]
    pub fn stack_instance_summaries(&self) -> terrars::ListRef<CloudformationStackSetInstance_StackInstanceSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stack_instance_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_instance_region` after provisioning.\n"]
    pub fn stack_set_instance_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_instance_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_name` after provisioning.\n"]
    pub fn stack_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_targets` after provisioning.\n"]
    pub fn deployment_targets(&self) -> terrars::ListRef<CloudformationStackSetInstance_DeploymentTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> terrars::ListRef<CloudformationStackSetInstance_OperationPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSetInstance_TimeoutsRef {
        CloudformationStackSetInstance_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudformationStackSetInstanceDynamic {
    deployment_targets: std::option::Option<terrars::DynamicBlock<CloudformationStackSetInstance_DeploymentTargets>>,
    operation_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<CloudformationStackSetInstance_OperationPreferences>>,
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstance_StackInstanceSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudformationStackSetInstance_StackInstanceSummaries {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_id`.\n"]
    pub fn set_organizational_unit_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack_id`.\n"]
    pub fn set_stack_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSetInstance_StackInstanceSummaries {
    type O = terrars::BlockAssignable<CloudformationStackSetInstance_StackInstanceSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstance_StackInstanceSummaries {}

impl BuildCloudformationStackSetInstance_StackInstanceSummaries {
    pub fn build(self) -> CloudformationStackSetInstance_StackInstanceSummaries {
        CloudformationStackSetInstance_StackInstanceSummaries {
            account_id: core::default::Default::default(),
            organizational_unit_id: core::default::Default::default(),
            stack_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSetInstance_StackInstanceSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetInstance_StackInstanceSummariesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudformationStackSetInstance_StackInstanceSummariesRef {
        CloudformationStackSetInstance_StackInstanceSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstance_StackInstanceSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_id` after provisioning.\n"]
    pub fn organizational_unit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstance_DeploymentTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_filter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accounts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accounts_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudformationStackSetInstance_DeploymentTargets {
    #[doc = "Set the field `account_filter_type`.\n"]
    pub fn set_account_filter_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.account_filter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accounts_url`.\n"]
    pub fn set_accounts_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accounts_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_ids`.\n"]
    pub fn set_organizational_unit_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.organizational_unit_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSetInstance_DeploymentTargets {
    type O = terrars::BlockAssignable<CloudformationStackSetInstance_DeploymentTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstance_DeploymentTargets {}

impl BuildCloudformationStackSetInstance_DeploymentTargets {
    pub fn build(self) -> CloudformationStackSetInstance_DeploymentTargets {
        CloudformationStackSetInstance_DeploymentTargets {
            account_filter_type: core::default::Default::default(),
            accounts: core::default::Default::default(),
            accounts_url: core::default::Default::default(),
            organizational_unit_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSetInstance_DeploymentTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetInstance_DeploymentTargetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudformationStackSetInstance_DeploymentTargetsRef {
        CloudformationStackSetInstance_DeploymentTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstance_DeploymentTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_filter_type` after provisioning.\n"]
    pub fn account_filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_filter_type", self.base))
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `accounts_url` after provisioning.\n"]
    pub fn accounts_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accounts_url", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_ids` after provisioning.\n"]
    pub fn organizational_unit_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.organizational_unit_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstance_OperationPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    concurrency_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_concurrency_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_order: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudformationStackSetInstance_OperationPreferences {
    #[doc = "Set the field `concurrency_mode`.\n"]
    pub fn set_concurrency_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.concurrency_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_count`.\n"]
    pub fn set_failure_tolerance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_percentage`.\n"]
    pub fn set_failure_tolerance_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_count`.\n"]
    pub fn set_max_concurrent_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_percentage`.\n"]
    pub fn set_max_concurrent_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_concurrency_type`.\n"]
    pub fn set_region_concurrency_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.region_concurrency_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_order`.\n"]
    pub fn set_region_order(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.region_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSetInstance_OperationPreferences {
    type O = terrars::BlockAssignable<CloudformationStackSetInstance_OperationPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstance_OperationPreferences {}

impl BuildCloudformationStackSetInstance_OperationPreferences {
    pub fn build(self) -> CloudformationStackSetInstance_OperationPreferences {
        CloudformationStackSetInstance_OperationPreferences {
            concurrency_mode: core::default::Default::default(),
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrent_count: core::default::Default::default(),
            max_concurrent_percentage: core::default::Default::default(),
            region_concurrency_type: core::default::Default::default(),
            region_order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSetInstance_OperationPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetInstance_OperationPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudformationStackSetInstance_OperationPreferencesRef {
        CloudformationStackSetInstance_OperationPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstance_OperationPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrency_mode` after provisioning.\n"]
    pub fn concurrency_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrency_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_count` after provisioning.\n"]
    pub fn max_concurrent_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_percentage` after provisioning.\n"]
    pub fn max_concurrent_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `region_concurrency_type` after provisioning.\n"]
    pub fn region_concurrency_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_concurrency_type", self.base))
    }

    #[doc = "Get a reference to the value of field `region_order` after provisioning.\n"]
    pub fn region_order(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_order", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSetInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudformationStackSetInstance_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSetInstance_Timeouts {
    type O = terrars::BlockAssignable<CloudformationStackSetInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSetInstance_Timeouts {}

impl BuildCloudformationStackSetInstance_Timeouts {
    pub fn build(self) -> CloudformationStackSetInstance_Timeouts {
        CloudformationStackSetInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSetInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudformationStackSetInstance_TimeoutsRef {
        CloudformationStackSetInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSetInstance_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
