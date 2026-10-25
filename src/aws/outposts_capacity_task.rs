#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OutpostsCapacityTaskData_ {
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
    asset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_id: std::option::Option<terrars::PrimField<std::string::String>>,
    outpost_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_action_on_blocking_instances: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_pool: std::option::Option<std::vec::Vec<OutpostsCapacityTask_InstancePool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances_to_exclude: std::option::Option<std::vec::Vec<OutpostsCapacityTask_InstancesToExclude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OutpostsCapacityTask_Timeouts>,
    dynamic: OutpostsCapacityTaskDynamic,
}

struct OutpostsCapacityTask_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OutpostsCapacityTaskData_>,
}

#[derive(Clone)]
pub struct OutpostsCapacityTask(std::rc::Rc<OutpostsCapacityTask_>);

impl OutpostsCapacityTask {
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

    #[doc = "Set the field `asset_id`.\n"]
    pub fn set_asset_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().asset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order_id`.\n"]
    pub fn set_order_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_action_on_blocking_instances`.\n"]
    pub fn set_task_action_on_blocking_instances(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().task_action_on_blocking_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_pool`.\n"]
    pub fn set_instance_pool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OutpostsCapacityTask_InstancePool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instances_to_exclude`.\n"]
    pub fn set_instances_to_exclude(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OutpostsCapacityTask_InstancesToExclude>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instances_to_exclude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instances_to_exclude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OutpostsCapacityTask_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `asset_id` after provisioning.\n"]
    pub fn asset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_task_id` after provisioning.\n"]
    pub fn capacity_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `completion_date` after provisioning.\n"]
    pub fn completion_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `order_id` after provisioning.\n"]
    pub fn order_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_identifier` after provisioning.\n"]
    pub fn outpost_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_action_on_blocking_instances` after provisioning.\n"]
    pub fn task_action_on_blocking_instances(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_action_on_blocking_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_pool` after provisioning.\n"]
    pub fn instance_pool(&self) -> terrars::ListRef<OutpostsCapacityTask_InstancePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances_to_exclude` after provisioning.\n"]
    pub fn instances_to_exclude(&self) -> terrars::ListRef<OutpostsCapacityTask_InstancesToExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_to_exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OutpostsCapacityTask_TimeoutsRef {
        OutpostsCapacityTask_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OutpostsCapacityTask {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OutpostsCapacityTask { }

impl terrars::ToListMappable for OutpostsCapacityTask {
    type O = terrars::ListRef<OutpostsCapacityTaskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OutpostsCapacityTask_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_outposts_capacity_task".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOutpostsCapacityTask {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub outpost_identifier: terrars::PrimField<std::string::String>,
}

impl BuildOutpostsCapacityTask {
    pub fn build(self, stack: &mut terrars::Stack) -> OutpostsCapacityTask {
        let out = OutpostsCapacityTask(std::rc::Rc::new(OutpostsCapacityTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OutpostsCapacityTaskData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                asset_id: core::default::Default::default(),
                order_id: core::default::Default::default(),
                outpost_identifier: self.outpost_identifier,
                region: core::default::Default::default(),
                task_action_on_blocking_instances: core::default::Default::default(),
                instance_pool: core::default::Default::default(),
                instances_to_exclude: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OutpostsCapacityTaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OutpostsCapacityTaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OutpostsCapacityTaskRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asset_id` after provisioning.\n"]
    pub fn asset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_task_id` after provisioning.\n"]
    pub fn capacity_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `completion_date` after provisioning.\n"]
    pub fn completion_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `order_id` after provisioning.\n"]
    pub fn order_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_identifier` after provisioning.\n"]
    pub fn outpost_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_action_on_blocking_instances` after provisioning.\n"]
    pub fn task_action_on_blocking_instances(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_action_on_blocking_instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_pool` after provisioning.\n"]
    pub fn instance_pool(&self) -> terrars::ListRef<OutpostsCapacityTask_InstancePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances_to_exclude` after provisioning.\n"]
    pub fn instances_to_exclude(&self) -> terrars::ListRef<OutpostsCapacityTask_InstancesToExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_to_exclude", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OutpostsCapacityTask_TimeoutsRef {
        OutpostsCapacityTask_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OutpostsCapacityTaskDynamic {
    instance_pool: std::option::Option<terrars::DynamicBlock<OutpostsCapacityTask_InstancePool>>,
    instances_to_exclude: std::option::Option<terrars::DynamicBlock<OutpostsCapacityTask_InstancesToExclude>>,
}

#[derive(Serialize)]
pub struct OutpostsCapacityTask_InstancePool {
    count: terrars::PrimField<f64>,
    instance_type: terrars::PrimField<std::string::String>,
}

impl OutpostsCapacityTask_InstancePool { }

impl terrars::ToListMappable for OutpostsCapacityTask_InstancePool {
    type O = terrars::BlockAssignable<OutpostsCapacityTask_InstancePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOutpostsCapacityTask_InstancePool {
    #[doc = ""]
    pub count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildOutpostsCapacityTask_InstancePool {
    pub fn build(self) -> OutpostsCapacityTask_InstancePool {
        OutpostsCapacityTask_InstancePool {
            count: self.count,
            instance_type: self.instance_type,
        }
    }
}

#[allow(dead_code)]
pub struct OutpostsCapacityTask_InstancePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OutpostsCapacityTask_InstancePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OutpostsCapacityTask_InstancePoolRef {
        OutpostsCapacityTask_InstancePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OutpostsCapacityTask_InstancePoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OutpostsCapacityTask_InstancesToExclude {
    instances: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl OutpostsCapacityTask_InstancesToExclude { }

impl terrars::ToListMappable for OutpostsCapacityTask_InstancesToExclude {
    type O = terrars::BlockAssignable<OutpostsCapacityTask_InstancesToExclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOutpostsCapacityTask_InstancesToExclude {
    #[doc = ""]
    pub instances: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildOutpostsCapacityTask_InstancesToExclude {
    pub fn build(self) -> OutpostsCapacityTask_InstancesToExclude {
        OutpostsCapacityTask_InstancesToExclude { instances: self.instances }
    }
}

#[allow(dead_code)]
pub struct OutpostsCapacityTask_InstancesToExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OutpostsCapacityTask_InstancesToExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OutpostsCapacityTask_InstancesToExcludeRef {
        OutpostsCapacityTask_InstancesToExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OutpostsCapacityTask_InstancesToExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct OutpostsCapacityTask_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OutpostsCapacityTask_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OutpostsCapacityTask_Timeouts {
    type O = terrars::BlockAssignable<OutpostsCapacityTask_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOutpostsCapacityTask_Timeouts {}

impl BuildOutpostsCapacityTask_Timeouts {
    pub fn build(self) -> OutpostsCapacityTask_Timeouts {
        OutpostsCapacityTask_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OutpostsCapacityTask_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OutpostsCapacityTask_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OutpostsCapacityTask_TimeoutsRef {
        OutpostsCapacityTask_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OutpostsCapacityTask_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
